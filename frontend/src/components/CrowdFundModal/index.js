import {
  Button,
  Card,
  CardContent,
  CardHeader,
  Dialog,
  DialogContent,
  DialogTitle,
  Grid,
  IconButton,
  LinearProgress,
  linearProgressClasses,
  styled,
  TextField,
  Typography,
} from "@mui/material";
import { useState, useEffect } from "react";
import CloseIcon from "@mui/icons-material/Close";
import Image from "next/image";
import { Box, Stack } from "@mui/system";
const BootstrapDialog = styled(Dialog)(({ theme }) => ({
  "& .MuiDialogContent-root": {
    padding: theme.spacing(4),
  },
  "& .MuiPaper-root": {
    background: "#040d1f",
  },
  "& .MuiDialogActions-root": {
    padding: theme.spacing(1),
  },
}));
const BootstrapDialogTitle = (props) => {
  const { children, onClose, ...other } = props;

  return (
    <DialogTitle sx={{ m: 0, p: 2 }} {...other}>
      {children}
      {onClose ? (
        <IconButton
          aria-label="close"
          onClick={onClose}
          sx={{
            position: "absolute",
            right: 8,
            top: 8,
            color: (theme) => theme.palette.grey[500],
          }}
        >
          <CloseIcon />
        </IconButton>
      ) : null}
    </DialogTitle>
  );
};
export function useCrowdFundModal() {
  const [open, setOpen] = useState(false);
  const handleClickOpen = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };
  return { open, handleClickOpen, handleClose };
}

export default function CrowdFundModal({ item, index, open, handleClose, wallet, progress, goal }) {
  const [fundEnabled, setFundEnabled] = useState(false);
  const [fundMsg, setFundMsg] = useState("Please connect wallet.");
  const [goalReached, setGoalReached] = useState(false);

  useEffect(() => {
    const loadProgresses = async () => {
      if (wallet) {
        if (progress != undefined && (progress == goal)) {
          setFundEnabled(false);
          setFundMsg("Goal reached");
          setGoalReached(true);
        } else if (wallet.walletSignedIn()) {
          setFundEnabled(true);
          setFundMsg("Contribute")
        } else {
          setFundEnabled(false);
          setFundMsg("Please connect wallet.");
        }
      }
    }

    loadProgresses();
  }, [open])

  const fundUSDC = () => {
    let amount = document.getElementById("fundAmount").value;
    wallet.fundUSDC(index.toString(), amount);
  }

  return (
    <BootstrapDialog open={open} onClose={handleClose} fullWidth maxWidth="md">
      <BootstrapDialogTitle onClose={handleClose}></BootstrapDialogTitle>
      <DialogContent>
        <Grid container>
          <Grid item md={6} xs={12}>
            <Typography variant="h1" component="h1">
              {item.title}
            </Typography>
            <Typography variant="body3" component={"p"} sx={{ mt: 2 }}>
              {"$"}{goal}
            </Typography>
            <Typography variant="body2" sx={{ mt: 2 }}>
              {item.description}
            </Typography>
            <Stack direction={"row"} sx={{ mt: 3 }}>
              <svg
                width="56"
                height="56"
                viewBox="0 0 56 56"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                transform="scale(0.75)"
              >
                <path
                  d="M17.6265 32.625H30.436C30.5183 32.6251 30.5998 32.6089 30.6758 32.5774C30.7519 32.546 30.821 32.4998 30.8792 32.4417C30.9373 32.3835 30.9835 32.3144 31.0149 32.2383C31.0464 32.1623 31.0626 32.0808 31.0625 31.9985V30.1265C31.0626 30.0442 31.0464 29.9627 31.0149 29.8867C30.9835 29.8106 30.9373 29.7415 30.8792 29.6833C30.821 29.6252 30.7519 29.579 30.6758 29.5476C30.5998 29.5161 30.5183 29.4999 30.436 29.5H17.6265C17.5442 29.4999 17.4627 29.5161 17.3867 29.5476C17.3106 29.579 17.2415 29.6252 17.1833 29.6833C17.1252 29.7415 17.079 29.8106 17.0476 29.8867C17.0161 29.9627 16.9999 30.0442 17 30.1265V31.9985C16.9999 32.0808 17.0161 32.1623 17.0476 32.2383C17.079 32.3144 17.1252 32.3835 17.1833 32.4417C17.2415 32.4998 17.3106 32.546 17.3867 32.5774C17.4627 32.6089 17.5442 32.6251 17.6265 32.625ZM17.6265 20.125H30.436C30.5183 20.1251 30.5998 20.1089 30.6758 20.0774C30.7519 20.046 30.821 19.9998 30.8792 19.9417C30.9373 19.8835 30.9835 19.8144 31.0149 19.7383C31.0464 19.6623 31.0626 19.5808 31.0625 19.4985V17.6265C31.0626 17.5442 31.0464 17.4627 31.0149 17.3867C30.9835 17.3106 30.9373 17.2415 30.8792 17.1833C30.821 17.1252 30.7519 17.079 30.6758 17.0476C30.5998 17.0161 30.5183 16.9999 30.436 17H17.6265C17.5442 16.9999 17.4627 17.0161 17.3867 17.0476C17.3106 17.079 17.2415 17.1252 17.1833 17.1833C17.1252 17.2415 17.079 17.3106 17.0476 17.3867C17.0161 17.4627 16.9999 17.5442 17 17.6265V19.4985C16.9999 19.5808 17.0161 19.6623 17.0476 19.7383C17.079 19.8144 17.1252 19.8835 17.1833 19.9417C17.2415 19.9998 17.3106 20.046 17.3867 20.0774C17.4627 20.1089 17.5442 20.1251 17.6265 20.125ZM38.0938 23.25H17.7813C17.574 23.25 17.3753 23.3323 17.2288 23.4788C17.0823 23.6253 17 23.824 17 24.0312V25.5938C17 25.801 17.0823 25.9997 17.2288 26.1462C17.3753 26.2927 17.574 26.375 17.7813 26.375H38.0938C38.301 26.375 38.4997 26.2927 38.6462 26.1462C38.7927 25.9997 38.875 25.801 38.875 25.5938V24.0312C38.875 23.824 38.7927 23.6253 38.6462 23.4788C38.4997 23.3323 38.301 23.25 38.0938 23.25ZM38.0938 35.75H17.7813C17.574 35.75 17.3753 35.8323 17.2288 35.9788C17.0823 36.1253 17 36.324 17 36.5312V38.0938C17 38.301 17.0823 38.4997 17.2288 38.6462C17.3753 38.7927 17.574 38.875 17.7813 38.875H38.0938C38.301 38.875 38.4997 38.7927 38.6462 38.6462C38.7927 38.4997 38.875 38.301 38.875 38.0938V36.5312C38.875 36.324 38.7927 36.1253 38.6462 35.9788C38.4997 35.8323 38.301 35.75 38.0938 35.75Z"
                  fill="white"
                />
                <circle
                  cx="28"
                  cy="28"
                  r="28"
                  fill="#D9D9D9"
                  fill-opacity="0.2"
                />
              </svg>
              <svg
                width="56"
                height="56"
                viewBox="0 0 56 56"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                transform="scale(0.75)"
              >
                <path
                  d="M40.6 18H21C20.6287 18 20.2726 18.1475 20.0101 18.4101C19.7475 18.6726 19.6 19.0287 19.6 19.4V23.6H26.6V20.8H35V34.8H40.6C40.9713 34.8 41.3274 34.6525 41.59 34.3899C41.8525 34.1274 42 33.7713 42 33.4V19.4C42 19.0287 41.8525 18.6726 41.59 18.4101C41.3274 18.1475 40.9713 18 40.6 18V18ZM24.15 22.5062C24.15 22.6107 24.1085 22.7108 24.0347 22.7847C23.9608 22.8585 23.8607 22.9 23.7562 22.9H22.4437C22.3393 22.9 22.2392 22.8585 22.1653 22.7847C22.0915 22.7108 22.05 22.6107 22.05 22.5062V21.1937C22.05 21.0893 22.0915 20.9892 22.1653 20.9153C22.2392 20.8415 22.3393 20.8 22.4437 20.8H23.7562C23.8607 20.8 23.9608 20.8415 24.0347 20.9153C24.1085 20.9892 24.15 21.0893 24.15 21.1937V22.5062ZM39.55 31.6062C39.55 31.7107 39.5085 31.8108 39.4347 31.8847C39.3608 31.9585 39.2607 32 39.1562 32H37.8438C37.7393 32 37.6392 31.9585 37.5653 31.8847C37.4915 31.8108 37.45 31.7107 37.45 31.6062V30.2937C37.45 30.1893 37.4915 30.0892 37.5653 30.0153C37.6392 29.9415 37.7393 29.9 37.8438 29.9H39.1562C39.2607 29.9 39.3608 29.9415 39.4347 30.0153C39.5085 30.0892 39.55 30.1893 39.55 30.2937V31.6062ZM39.55 27.0562C39.55 27.1607 39.5085 27.2608 39.4347 27.3347C39.3608 27.4085 39.2607 27.45 39.1562 27.45H37.8438C37.7393 27.45 37.6392 27.4085 37.5653 27.3347C37.4915 27.2608 37.45 27.1607 37.45 27.0562V25.7438C37.45 25.6393 37.4915 25.5392 37.5653 25.4653C37.6392 25.3915 37.7393 25.35 37.8438 25.35H39.1562C39.2607 25.35 39.3608 25.3915 39.4347 25.4653C39.5085 25.5392 39.55 25.6393 39.55 25.7438V27.0562ZM39.55 22.5062C39.55 22.6107 39.5085 22.7108 39.4347 22.7847C39.3608 22.8585 39.2607 22.9 39.1562 22.9H37.8438C37.7393 22.9 37.6392 22.8585 37.5653 22.7847C37.4915 22.7108 37.45 22.6107 37.45 22.5062V21.1937C37.45 21.0893 37.4915 20.9892 37.5653 20.9153C37.6392 20.8415 37.7393 20.8 37.8438 20.8H39.1562C39.2607 20.8 39.3608 20.8415 39.4347 20.9153C39.5085 20.9892 39.55 21.0893 39.55 21.1937V22.5062ZM32.2 25H15.4C15.0287 25 14.6726 25.1475 14.4101 25.4101C14.1475 25.6726 14 26.0287 14 26.4V39C14 39.3713 14.1475 39.7274 14.4101 39.9899C14.6726 40.2525 15.0287 40.4 15.4 40.4H32.2C32.5713 40.4 32.9274 40.2525 33.1899 39.9899C33.4525 39.7274 33.6 39.3713 33.6 39V26.4C33.6 26.0287 33.4525 25.6726 33.1899 25.4101C32.9274 25.1475 32.5713 25 32.2 25ZM18.2 27.8C18.4769 27.8 18.7476 27.8821 18.9778 28.0359C19.208 28.1898 19.3875 28.4084 19.4934 28.6642C19.5994 28.9201 19.6271 29.2016 19.5731 29.4731C19.5191 29.7447 19.3857 29.9942 19.1899 30.1899C18.9942 30.3857 18.7447 30.5191 18.4731 30.5731C18.2016 30.6271 17.9201 30.5994 17.6642 30.4934C17.4084 30.3875 17.1898 30.208 17.0359 29.9778C16.8821 29.7476 16.8 29.4769 16.8 29.2C16.8 28.8287 16.9475 28.4726 17.2101 28.2101C17.4726 27.9475 17.8287 27.8 18.2 27.8ZM30.8 37.6H16.8V36.2L19.6 33.4L21 34.8L26.6 29.2L30.8 33.4V37.6Z"
                  fill="white"
                />
                <circle
                  cx="28"
                  cy="28"
                  r="28"
                  fill="#D9D9D9"
                  fill-opacity="0.2"
                />
              </svg>
              <svg
                width="56"
                height="56"
                viewBox="0 0 56 56"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                transform="scale(0.75)"
              >
                <path
                  d="M30.9375 20.6406V14H21.1719C20.5225 14 20 14.5225 20 15.1719V37.8281C20 38.4775 20.5225 39 21.1719 39H37.5781C38.2275 39 38.75 38.4775 38.75 37.8281V21.8125H32.1094C31.4648 21.8125 30.9375 21.2852 30.9375 20.6406ZM34.0625 32.1641C34.0625 32.4863 33.7988 32.75 33.4766 32.75H25.2734C24.9512 32.75 24.6875 32.4863 24.6875 32.1641V31.7734C24.6875 31.4512 24.9512 31.1875 25.2734 31.1875H33.4766C33.7988 31.1875 34.0625 31.4512 34.0625 31.7734V32.1641ZM34.0625 29.0391C34.0625 29.3613 33.7988 29.625 33.4766 29.625H25.2734C24.9512 29.625 24.6875 29.3613 24.6875 29.0391V28.6484C24.6875 28.3262 24.9512 28.0625 25.2734 28.0625H33.4766C33.7988 28.0625 34.0625 28.3262 34.0625 28.6484V29.0391ZM34.0625 25.5234V25.9141C34.0625 26.2363 33.7988 26.5 33.4766 26.5H25.2734C24.9512 26.5 24.6875 26.2363 24.6875 25.9141V25.5234C24.6875 25.2012 24.9512 24.9375 25.2734 24.9375H33.4766C33.7988 24.9375 34.0625 25.2012 34.0625 25.5234ZM38.75 19.9521V20.25H32.5V14H32.7979C33.1104 14 33.4082 14.1221 33.6279 14.3418L38.4082 19.127C38.6279 19.3467 38.75 19.6445 38.75 19.9521Z"
                  fill="white"
                />
                <circle
                  cx="28"
                  cy="28"
                  r="28"
                  fill="#D9D9D9"
                  fill-opacity="0.2"
                />
              </svg>
              <svg
                width="56"
                height="56"
                viewBox="0 0 56 56"
                fill="none"
                xmlns="http://www.w3.org/2000/svg"
                transform="scale(0.75)"
              >
                <path
                  d="M41.6737 25.7017L35.4236 31.6039C34.7648 32.2261 33.6667 31.7648 33.6667 30.8466V27.7232C27.3917 27.7653 24.7445 29.2472 26.515 35.1604C26.7095 35.8103 25.9574 36.3135 25.4296 35.9126C23.7384 34.6278 22.2083 32.1705 22.2083 29.6901C22.2083 23.4428 27.3125 22.2031 33.6667 22.1678V19.0422C33.6667 18.1232 34.7655 17.6634 35.4236 18.2849L41.6737 24.1872C42.1086 24.5979 42.1089 25.2907 41.6737 25.7017ZM33.6667 34.4552V37.4445H19.7778V23.5556H21.9877C22.0578 23.5555 22.1272 23.5413 22.1917 23.5138C22.2562 23.4862 22.3145 23.4459 22.363 23.3953C23.012 22.7196 23.7622 22.1848 24.5772 21.76C25.0602 21.5083 24.8812 20.7778 24.3365 20.7778H19.0833C17.9327 20.7778 17 21.7105 17 22.8611V38.1389C17 39.2895 17.9327 40.2222 19.0833 40.2222H34.3611C35.5117 40.2222 36.4444 39.2895 36.4444 38.1389V34.2845C36.4444 33.9247 36.0887 33.674 35.7495 33.7939C35.274 33.9621 34.7648 34.0124 34.2656 33.9405C33.9501 33.8951 33.6667 34.1364 33.6667 34.4552Z"
                  fill="white"
                />
                <circle
                  cx="28"
                  cy="28"
                  r="28"
                  fill="#D9D9D9"
                  fill-opacity="0.2"
                />
              </svg>
            </Stack>
          </Grid>
          <Grid item xs={12} md={6}>
            <Box display="flex" justifyContent="center">
              <div style={{borderRadius: '20px', overflow: 'hidden', width: 230, height: 200}}>
                <Image
                  src={item.reference != null ? item.media : "/" + item.extra + ".png"}
                  width={230}
                  height={200}
                  objectFit="cover"
                  layout="intrinsic"
                  alt={"image"}
                />
              </div>
            </Box>
          </Grid>
        </Grid>
        <Grid container spacing={2} mt={2}>
          <Grid item xs={12} md={6}>
            <Card
              variant="outlined"
              sx={{
                borderRadius: "15px",
                border: "1px solid rgba(255, 255, 255, 0.1)",
                background: "#0B101A",
              }}
            >
              <CardHeader title="Contributors">
                {/* <Typography color="#fff">Contributors</Typography> */}
              </CardHeader>
            </Card>
          </Grid>
          <Grid item xs={12} md={6}>
            <Card
              variant="outlined"
              sx={{
                borderRadius: "15px",
                border: "1px solid rgba(255, 255, 255, 0.1)",
                background: "#0B101A",
              }}
            >
              <CardHeader title="Progress"></CardHeader>
              <CardContent>
                <Box mb={5}>
                  <Box display="flex" justifyContent="space-between">
                    <Typography>${progress}</Typography>
                    <Typography>${goal}</Typography>
                  </Box>
                  <BorderLinearProgress variant="determinate" value={(progress / goal) * 100} />
                </Box>
                <Box display="flex" justifyContent="space-between">
                  <TextField
                    id="fundAmount"
                    label="$$$"
                    type="number"
                    InputLabelProps={{
                      shrink: true,
                    }}
                    variant="outlined"
                    color="tertiary"
                    focused
                  />
                  <Button sx={{"margin-left": "15px"}} variant="contained" disabled={!fundEnabled} onClick={() => fundUSDC()}>{fundMsg}</Button>
                </Box>
                {goalReached ?
                  (<Box mt={5} display="flex" justifyContent="center">
                    <Button variant="contained" onClick={() => wallet.claimTokens(item.extra)}>Claim Tokens</Button>
                   </Box>
                  ) : ""
                }
              </CardContent>
            </Card>
          </Grid>
        </Grid>
      </DialogContent>
    </BootstrapDialog>
  );
}

const BorderLinearProgress = styled(LinearProgress)(({ theme }) => ({
  height: 20,
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
