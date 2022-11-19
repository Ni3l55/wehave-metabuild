import { Button, InputBase, styled, alpha } from "@mui/material";
import { Box } from "@mui/system";
import React from "react";
import SearchIcon from "@mui/icons-material/Search";
const Search = styled("div")(({ theme }) => ({
  position: "relative",
  borderRadius: theme.shape.borderRadius,
  backgroundColor: "rgba(217, 217, 217, 0.2)",
  "&:hover": {
    backgroundColor: "rgba(217, 217, 217, 0.3)",
  },
  marginRight: theme.spacing(2),
  marginLeft: 0,
  width: "100%",
  display: "none",
  flexGrow: 1,
  [theme.breakpoints.up("sm")]: {
    display: "block",
    marginRight: theme.spacing(8),
    marginLeft: theme.spacing(3),
    width: "auto",
  },
}));

const SearchIconWrapper = styled("div")(({ theme }) => ({
  padding: theme.spacing(0, 2),
  height: "100%",
  position: "absolute",
  right: 0,
  pointerEvents: "none",
  display: "flex",
  alignItems: "center",
  justifyContent: "center",
}));

const StyledInputBase = styled(InputBase)(({ theme }) => ({
  color: "inherit",
  "& .MuiInputBase-input": {
    padding: theme.spacing(1, 1, 0, 5),
    // vertical padding + font size from searchIcon
    paddingRight: `calc(1em + ${theme.spacing(4)})`,
    transition: theme.transitions.create("width"),
    width: "100%",
    [theme.breakpoints.up("md")]: {
      width: "20ch",
    },
  },
}));

function trySignIn(wallet) {
  if (typeof window !== 'undefined') {
    wallet.signIn();
  }
}

const NavBar = (props) => {
  return (
    <Box
      display="flex"
      sx={{
        justifyContent: { xs: "flex-end", md: "space-between" },
        width: "100%",
      }}
    >
      <Search>
        <SearchIconWrapper>
          <SearchIcon />
        </SearchIconWrapper>
        <StyledInputBase
          placeholder="Search…"
          inputProps={{ "aria-label": "search" }}
        />
      </Search>
      <Button variant="contained" onClick={() => trySignIn(props.wallet)}>{props.wallet.walletSignedIn() ? props.wallet.accountId : "Connect Wallet" }</Button>
    </Box>
  );
};

export default NavBar;
