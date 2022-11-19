import {
  Box,
  Button,
  Container,
  Divider,
  Grid,
  Typography,
  LinearProgress,
  linearProgressClasses,
  styled,
  TextField,
} from "@mui/material";
import Image from "next/image";
import React from "react";
import ProgressBar from "react-bootstrap/ProgressBar";

import { useState, useEffect } from "react";

export const Proposal = (props) => {
  return (
    <>
      <Box
        mt={4}
        display="flex"
        bgcolor="background.secondary"
        sx={{
          borderRadius: "20px",
          p: "15px",
          justifyContent: "space-between",
          alignItems: "center",
          border: "1px solid rgba(255, 255, 255, 0.1)",
        }}
      >
        <Box display="flex" sx={{ alignItems: "center" }}>
          <Image
            src={"/" + props.ftAccount + ".png"}
            width={190}
            height={160}
            alt={"image"}
          />
          <Box mx={2}>
            <Typography variant="h2" color="text.secondary" sx={{ my: "4px" }}>
              {props.question}
            </Typography>
            <Box display="flex" justifyContent="space-between">
              <Typography>%0</Typography>
              <Typography>%100</Typography>
            </Box>
            <Box mt={2} width={500}>
              <ProgressBar>
                <ProgressBar variant="success" now={props.votes != undefined ? props.votes[0] : 0} key={1} label="yes" />
                <ProgressBar variant="danger" now={props.votes != undefined ? props.votes[1] : 0} key={2} label="no" />
              </ProgressBar>
            </Box>
            <Box mt={2}>
              <Button
                variant="tradeBorder"
                sx={{ my: "4px" }}
                onClick={() => props.wallet.voteForProposal(props.daoContract, props.proposalIndex, 0)}
                disabled={props.userVote == "0" ? "true" : ""}
              >
                Yes
              </Button>
              <Button
                variant="tradeBorder"
                sx={{ my: "4px" }}
                onClick={() => props.wallet.voteForProposal(props.daoContract, props.proposalIndex, 1)}
                disabled={props.userVote == "1" ? "true" : ""}
              >
                No
              </Button>
            </Box>
          </Box>
        </Box>
      </Box>
    </>
  );
};

const Govern = (props) => {
  const [items, setItems] = useState([]) // Keep the different items
  const [proposals, setProposals] = useState({}); // Keep proposals per item
  const [voteProgress, setVoteProgress] = useState({}); // Keep vote progress per proposal per item
  const [userVotes, setUserVotes] = useState({}); // Per proposal per item, the votes that this user made

  // Get proposals, votes, supply, vote weight // TODO images
  useEffect(() => {
    if (props.wallet) {
      const fetchAllItemProposals = async () => {
          for (var i of [0,1,2,3,4,5,6,7,8,9,10]) {       // TODO instead of going over 1-10, get a list of used token id's
            // Get the token on this tokenId
            let token = await props.wallet.getSingleTokenfromNFT(i);

            if (token != undefined) {
              setItems(oldItems => [...oldItems, token]);

              console.log(token);
              // Get the owner of the item (FT account)
              let ft_item_account = token.owner_id;

              // Prepend dao- to get the item DAO
              let dao_account = "dao-" + ft_item_account;

              let itemProposals = await props.wallet.getProposals(dao_account);
              console.log(itemProposals);
              setProposals(oldProposals => ({...oldProposals, [i]: itemProposals}));

              // Asynchronously also get the votes
              fetchProposalVotes(i, ft_item_account, dao_account, itemProposals);
            }
          }
      }
      setProposals({});
      setVoteProgress({});
      setUserVotes({});
      fetchAllItemProposals();
    }
  }, []);

  const fetchProposalVotes = async (itemIndex, FT, dao, itemProposals) => {
    // Get token total supply for calculating vote weight
    let itemTokenSupply = await props.wallet.getFTTotalSupply(FT);

    console.log("Iterating over proposals");
    console.log(proposals[itemIndex]);

    for (let proposal in itemProposals) {
      // Get all votes in the proposal
      console.log("Fetching votes for proposal " + proposal);
      let proposalVotes = await props.wallet.getProposalVotes(dao, parseInt(proposal));

      // Calculate the votes
      let votePercentages = [];

      // TODO this calculation off-chain in backend or on-chain
      for (let proposalVote in proposalVotes) {
        console.log("Calculating vote progress weight for vote " + proposalVote);
        let user = proposalVotes[proposalVote][0];
        let userTokenBalance = await props.wallet.getFTUserBalance(FT, user);
        let userVoteWeight = (userTokenBalance / itemTokenSupply) * 100;

        if (votePercentages[proposalVotes[proposalVote][1]] == undefined) {
          votePercentages[proposalVotes[proposalVote][1]] = userVoteWeight;
        } else {
          votePercentages[proposalVotes[proposalVote][1]] += userVoteWeight;
        }

        // Also keep track which one this user voted for
        if (user == props.wallet.accountId) {
          setUserVotes(oldUserVotes => ({...oldUserVotes, [[itemIndex, proposal]]: proposalVotes[proposalVote][1]}))
        }
      }

      setVoteProgress(oldVoteProgress => ({...oldVoteProgress, [[itemIndex, proposal]]: votePercentages}));
    }
  }

  const newProposal = async (daoContract, itemIndex) => {
    let question = document.getElementById(itemIndex.toString() + "_newQuestion").value;
    let options = ["yes", "no"]; // TODO change in the future

    await props.wallet.createProposal(daoContract, question, options);
  }

  const allProposals = items.map((item, i) => (
    <Box
      mt={4}
      bgcolor="background.secondary"
      sx={{
        borderRadius: "20px",
        p: "15px",
        border: "1px solid rgba(255, 255, 255, 0.1)",
      }}
      key={i}
    >
      <Box
        mb={2}
        display="flex"
        sx={{
          alignItems: "center",
          justifyContent: "space-between",
        }}
      >
        <Typography variant="h2" color="text.secondary" sx={{ my: "4px" }}>{items[i].metadata.title}</Typography>
        <Divider color="#FFF"></Divider>
      </Box>
        {(proposals[i] != undefined && proposals[i].length > 0) ? proposals[i].map((question, index) => (
          <Proposal key={i.toString() + "_" + index.toString()} ftAccount={item.metadata.extra} question={question} votes={voteProgress[[i, index]]} wallet={props.wallet} daoContract={"dao-" + item.owner_id} proposalIndex={index} userVote={userVotes[[i, index]]} />
        )) : <Box>No proposals found.</Box>}
        <Box display="flex" justifyContent="center" mt={10}>
          <TextField
            id={i.toString() + "_newQuestion"}
            label="Question"
            variant="outlined"
            color="tertiary"
            focused
            sx={{mr: 3}}
          />
          <TextField
            id={i.toString() + "_newOptions"}
            label="Options"
            color="tertiary"
            focused
            defaultValue="Yes, No"
            sx={{mr: 10}}
          />
          <Button variant="contained" onClick={() => newProposal("dao-" + item.owner_id, i)}>Propose</Button>
        </Box>
    </Box>
  ));

  return (
    <div>
      <Box mt={8}>
        <Container>
            <Grid item md={12}>
              <Box>
                {allProposals}
              </Box>
            </Grid>
        </Container>
      </Box>
    </div>
  );
}

export default Govern;
