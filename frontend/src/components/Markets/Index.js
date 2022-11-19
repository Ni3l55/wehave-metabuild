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
  TextField
} from "@mui/material";
import Image from "next/image";
import React from "react";
import AddPhotoAlternateIcon from '@mui/icons-material/AddPhotoAlternate';
import AddIcon from '@mui/icons-material/Add';
import CheckIcon from '@mui/icons-material/Check';
import Backdrop from '@mui/material/Backdrop';
import CircularProgress from '@mui/material/CircularProgress';

import { saveOnIPFS } from '../utils/nft-storage.js';

import CrowdFundModal, { useCrowdFundModal } from "../CrowdFundModal";
import { useState, useEffect } from "react";

const IPFS_HTTPS_PREFIX = "https://ipfs.io/ipfs/";

const TradeData = [
  {
    title: "Ensors' artwork",
    price: "$7 100 000",
    variation: "(+2.3%)",
    description: "Christ's Entry Into Brussels in 1889, painted by James Ensor. Valued at $7m.",
    img: "ensor_christ",
  },
  {
    title: "Nike MAG",
    price: "$21 000",
    variation: "",
    description: "Originally released for sale in 2011 and again in 2016. Both launches were in limited quantities.",
    img: "nikeMAG"
  }
];

const PtrsData = [
  {
    title: "Ferrari F40",
    price: "$1 500 000",
    variation: "",
    description:
      "The last Ferrari built under the supervision of Enzo Ferrari himself.",
    img: "PTR1",
  },
  {
    title: "Richard Mille RM25-01",
    price: "$950 000",
    variation: "(+3%)",
    description:
      "Manually winding, 72 hours of operation when winded. The definition of craftmanship.",
    img: "rm2501",
  },
  {
    title: "Charizard Pokemon Card",
    price: "$430 000",
    variation: "(+10%)",
    description:
      "The classic amongst the pokemon card collectibles: Charizard.",
    img: "charizard",
  },
  {
    title: "Darth Vader Helmet",
    price: "$300 000",
    variation: "(+85%)",
    description:
      "The original item from the movies.",
    img: "darthvader",
  },
];

const Markets = (props) => {
  const { open, handleClickOpen, handleClose } = useCrowdFundModal();
  const [loadingOverlay, setLoadingOverlay] = useState(false);
  const [items, setItems] = useState([]);
  const [crowdfunds, setCrowdfunds] = useState([]);
  const [crowdfundGoals, setCrowdfundGoals] = useState([]);
  const [crowdfundProgresses, setCrowdfundProgresses] = useState([]);
  const [showNewCrowdfund, setShowNewCrowdfund] = useState(false);
  const [newName, setNewName] = useState("");
  const [newAccountName, setNewAccountName] = useState("");
  const [uploadedFile, setUploadedFile] = useState();

  const handleFileUpload = (event) => {
      setUploadedFile(event.target.files[0]);
  }

  // Get all current items once on page load
  useEffect(() => {
    if (props.wallet) {
      const fetchItems = async () => {
        let currentItems = [];

        // Go over token 0-10 and check if exists
        // TODO get all registered items instead
        for (var i of [0,1,2,3,4,5,6,7,8,9,10]) {       // TODO instead of going over 1-10, get a list of used token id's
          // Get the token on this tokenId
          let token = await props.wallet.getSingleTokenfromNFT(i);

          if (token != undefined) {
            console.log(token);
            currentItems.push(token);
          }

        }

        setItems(currentItems);
      }

      fetchItems();
    }
  }, [])

  // Get all current crowdfunds once on page load
  useEffect(() => {
    if (props.wallet) {
      const fetchCrowdfunds = async () => {
        // Get all crowdfunds and update state
        let currentCrowdfunds = await props.wallet.getCurrentCrowdfunds();
        setCrowdfunds(currentCrowdfunds);
      }

      fetchCrowdfunds();
    }
  }, [])

  // Get all crowdfund progresses on crowdfund change
  useEffect(() => {
    if (crowdfunds) {
      const loadProgresses = async () => {
        if (props.wallet) {
          for (let crowdfund in crowdfunds) {
            let currentProgress = await props.wallet.getCrowdfundProgress(parseInt(crowdfund));
            let currentGoal = await props.wallet.getCrowdfundGoal(parseInt(crowdfund));

            setCrowdfundProgresses(oldProgresses => [...oldProgresses, currentProgress]);
            setCrowdfundGoals(oldGoals => [...oldGoals, currentGoal]);
          }
        }
      }

      loadProgresses();
    }
  }, [crowdfunds]);

  // Calculate the accountName from the name
  // TODO disregard account name
  useEffect(() => {
    if (newName) {
      setNewAccountName(newName.toLowerCase().replace(/ /g,''));
    }
  }, [newName])

  const newCrowdfund = async () => {
    let description = document.getElementById("newDescription").value;
    let goal = document.getElementById("newGoal").value;

    if (newName == "" || description == "" || newAccountName == "" || goal == 0 || goal == "") {
      alert("Not all crowdfund values have been filled.");
      return;
    }

    if (uploadedFile != undefined) {
      // TODO: allow for more metadata to be put in IPFS
      // Store a directory: information + picture instead of only picture

      setLoadingOverlay(true);

      try {
        let cid = await saveOnIPFS(newName, description, uploadedFile);  // Save picture on ipfs

        // TODO remove https part for ipfs supporting browsers
        let imgUrl = IPFS_HTTPS_PREFIX + cid + "/image.jpg";
        let metadataUrl = IPFS_HTTPS_PREFIX + cid + "/information.json";

        await props.wallet.createCrowdfund(newName, newAccountName, description, parseInt(goal), imgUrl, metadataUrl);  // Create crowdfund on-chain
        alert("Crowdfund was created successfully.");
      } catch (error) {
        alert(error);
      } finally {
        setLoadingOverlay(false);
      }
    } else {
      alert("You did not upload a picture.");
    }
  }

  return (
    <div>
    <Box>
        <Container>
          <Grid
            container
            spacing={{ xs: 6, sm: 10, md: 6 }}
            justifyContent={{ xs: "center", md: "default" }}
          >
            <Backdrop
              sx={{ color: '#fff', zIndex: (theme) => theme.zIndex.drawer + 1 }}
              open={loadingOverlay}
            >
              <CircularProgress color="inherit" />
            </Backdrop>
            <Grid item md={6} xs={12}>
                {items.map((ptdata, key) => (
                  <Box
                    mt={4}
                    bgcolor="background.secondary"
                    sx={{
                      borderRadius: "20px",
                      p: "0px",
                      border: "1px solid rgba(255, 255, 255, 0.1)",
                    }}
                    key={"box_" + key}
                  >
                    <Trade data={ptdata.metadata} key={key} />
                  </Box>
                ))}
            </Grid>
            <Grid item md={6} xs={12}>
                {crowdfunds.map((data, index) => (index > 16 && (
                  <Box
                    mt={4}
                    bgcolor="background.secondary"
                    sx={{
                      borderRadius: "20px",
                      p: "0px",
                      border: "1px solid rgba(255, 255, 255, 0.1)",
                    }}
                    key={"box_" + index}
                  >
                  <Crowdfund key={index} data={data} index={index} wallet={props.wallet} progress={crowdfundProgresses[index]} goal={crowdfundGoals[index]} />
                  </Box>
                )))}

                {showNewCrowdfund ?
                <Box bgcolor="background.secondary">
                  <Box display="flex" justifyContent="space-between" mt={5}>
                    <TextField
                      id="newName"
                      label="Name"
                      variant="outlined"
                      color="tertiary"
                      focused
                      sx={{mr: 1}}
                      onChange={(e) => setNewName(e.target.value)}
                    />
                    <TextField
                      id="newGoal"
                      label="Goal"
                      type="number"
                      variant="outlined"
                      color="tertiary"
                      focused
                      sx={{mr: 1}}
                    />
                  </Box>
                  <Box display="flex" justifyContent="space-between" mt={3}>
                    <TextField
                      id="newDescription"
                      label="Description"
                      color="tertiary"
                      multiline
                      rows={2}
                      focused
                      fullWidth
                      sx={{mr: 1}}
                    />
                    <Button
                      variant={uploadedFile == undefined ? "contained" : "containedApproved"}
                      component="label"
                    >
                      {uploadedFile == undefined ? <AddPhotoAlternateIcon fontSize="large" /> : <CheckIcon fontSize="large" />}
                      <input
                        type="file"
                        id="newPicture"
                        onChange={handleFileUpload}
                        hidden
                      />
                    </Button>
                  </Box>
                  <Box display="flex" justifyContent="center" mt={3}>
                    <Button variant="tradeBorderNoBg" onClick={() => setShowNewCrowdfund(false)} sx={{ mr: "10px" }}>Cancel</Button>
                    <Button
                      variant="tradeBorder"
                      onClick={() => newCrowdfund()}
                      disabled={!props.wallet.walletSignedIn()}>
                      {props.wallet.walletSignedIn() ? "Submit" : "Connect wallet"}
                    </Button>
                  </Box>
                </Box> :
                <Box display="flex" justifyContent="center" mt={5}><Button variant="tradeBorder" onClick={() => setShowNewCrowdfund(true)}><AddIcon /></Button></Box>
                }
            </Grid>
          </Grid>
        </Container>
      </Box>
    </div>
  );
};

export const Trade = (props) => {
  const { open, handleClickOpen, handleClose } = useCrowdFundModal();

  return (
    <>
      <CrowdFundModal item={props.data} index="0" open={open} handleClose={handleClose} wallet={props.wallet} />

      <Box
        display="flex"
        sx={{ justifyContent: "left", alignItems: "center" }}
      >
        <Box mr={2}>
          <div style={{borderRadius: '20px', overflow: 'hidden', width: 210, height: 185}}>
            <Image
              src={props.data.reference != null ? props.data.media : "/" + props.data.extra + ".png"}
              width={210}
              height={185}
              objectFit="cover"
              layout="intrinsic"
              alt={"image"}
            />
          </div>
        </Box>
        <Box>
          <Typography variant="h2" color="text.secondary" sx={{ my: "4px" }}>
            {props.data.title}
          </Typography>
          <Typography variant="body3" sx={{ my: "4px" }}>
            {props.data.price}{" "}
          </Typography>
          <Typography variant="body2" sx={{ my: "4px" }}>
            {props.data.description}
          </Typography>
          <Box display="flex">
            <Button variant="containedSmall" disabled sx={{ mt: "20px", mr: "10px" }}>
              Trade
            </Button>
          </Box>
        </Box>
      </Box>
    </>
  );
};

export const Crowdfund = (props) => {
  const { open, handleClickOpen, handleClose } = useCrowdFundModal();
  return (
    <>
      <CrowdFundModal item={props.data} index={props.index} wallet={props.wallet} open={open} handleClose={handleClose} progress={props.progress} goal={props.goal} />
      <Box
        display="flex"
        bgcolor="background.secondary"
        sx={{
          borderRadius: "20px",
          justifyContent: "left",
          alignItems: "center",
        }}
      >
          <Box>
            <div style={{borderRadius: '20px', overflow: 'hidden', width: 210, height: 185}}>
              <Image
                src={props.data.reference != null ? props.data.media : "/" + props.data.extra + ".png"}
                width={210}
                height={185}
                objectFit="cover"
                layout="intrinsic"
                alt={"image"}
              />
            </div>
          </Box>
          <Box mx={2}>
            <Typography variant="h2" color="text.secondary" sx={{ my: "4px" }}>
              {props.data.title}
            </Typography>
            <Typography variant="body3" sx={{ my: "4px" }}>
              {"$"}{props.goal}
            </Typography>
            <BorderLinearProgress sx={{ my: "10px"}} variant="determinate" value={(props.progress / props.goal) * 100} />
            <Box display="flex" justifyContent="right">
              <Button variant="trade" disabled sx={{ mt: "10px" }} mr={5}>
                Learn More
              </Button>
              <Button variant="containedSmall" sx={{ mt: "10px" }} mr={5} onClick={handleClickOpen}>
                Crowdfund
              </Button>
            </Box>
          </Box>
      </Box>
    </>
  );
};

const BorderLinearProgress = styled(LinearProgress)(({ theme }) => ({
  height: 15,
  width: 250,
  borderRadius: 15,
  [`&.${linearProgressClasses.colorPrimary}`]: {
    backgroundColor:
      theme.palette.grey[theme.palette.mode === "light" ? 200 : 800],
  },
  [`& .${linearProgressClasses.bar}`]: {
    borderRadius: 15,
    backgroundColor: theme.palette.mode === "light" ? "#64B5F6" : "#308fe8",
  },
}));

export default Markets;
