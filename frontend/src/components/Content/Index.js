import {
  Box,
  Button,
  Container,
  Divider,
  Grid,
  Typography,
} from "@mui/material";
import Image from "next/image";
import React from "react";
import Markets from "../Markets/Index";
import Trade from "../Trade/Index";
import Portfolio from "../Portfolio/Index";
import Govern from "../Govern/Index";

const Content = (props) => {
    if (props.selected == "Items") {
      return(<Markets wallet={props.wallet} />);
    } else if (props.selected == "Trade") {
      return(<Trade wallet={props.wallet} />);
    } else if (props.selected == "Portfolio") {
      return(<Portfolio wallet={props.wallet} />);
    } else if (props.selected == "Govern") {
      return(<Govern wallet={props.wallet} />);
    }
}

export default Content;
