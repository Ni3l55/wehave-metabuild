import { Box, Typography } from "@mui/material";
import dynamic from "next/dynamic";
import Head from "next/head";
import Image from "next/image";
import Content from "../components/Content/Index";
import ResponsiveDrawer from "../components/Drawer/Index";
import { useState } from "react";
import { Wallet } from '../components/utils/near-wallet';

// create the Wallet and the Contract
const crowdfundsContractId = "crowdfunds2-wehave.testnet";
const itemsContractId = "items2-wehave.testnet";
const wallet = new Wallet({crowdfundsContractId: crowdfundsContractId, itemsContractId: itemsContractId});

if (typeof window !== 'undefined') {
    window.onload = wallet.startUp();
}

export default function Home() {
  const [shownContent, setShownContent] = useState("Markets");

  return (
    <Box>
      <ResponsiveDrawer wallet={wallet} selectedCallback={setShownContent}>
        <Content wallet={wallet} selected={shownContent} />
      </ResponsiveDrawer>
    </Box>
  );
}
