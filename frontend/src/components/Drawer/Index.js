import * as React from "react";
import PropTypes from "prop-types";
import AppBar from "@mui/material/AppBar";
import Box from "@mui/material/Box";
import CssBaseline from "@mui/material/CssBaseline";
import Divider from "@mui/material/Divider";
import Drawer from "@mui/material/Drawer";
import IconButton from "@mui/material/IconButton";
import InboxIcon from "@mui/icons-material/MoveToInbox";
import List from "@mui/material/List";
import ListItem from "@mui/material/ListItem";
import ListItemButton from "@mui/material/ListItemButton";
import ListItemIcon from "@mui/material/ListItemIcon";
import ListItemText from "@mui/material/ListItemText";
import MailIcon from "@mui/icons-material/Mail";
import MenuIcon from "@mui/icons-material/Menu";
import Toolbar from "@mui/material/Toolbar";
import Typography from "@mui/material/Typography";
import Image from "next/image";
import NavBar from "../Navbar/Index";
import { useState, useEffect } from "react";

const drawerWidth = 300;

function ResponsiveDrawer(props) {
  const { window, children } = props;
  const wallet = props.wallet;
  const [mobileOpen, setMobileOpen] = useState(false);
  const [selected, setSelected] = useState("Items");

  // Propagate selection back up
  useEffect(() => {
    props.selectedCallback(selected);
  }, [selected]);

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  const drawer = (
    <Box
      bgcolor={"background.secondary"}
      sx={{
        border: "none",
        minHeight: "100vh",
      }}
    >
      <Box mt={3} display="flex" justifyContent={"center"}>
        <Image
          src="/icon.png"
          width={"200px"}
          height={"150px"}
          alt={"image"}
        ></Image>
      </Box>
      <Divider width="100%" sx={{ borderColor: "rgba(255, 255, 255, 0.2)" }} />
      <List sx={{ width: "100%", px: 3 }}>
        {["Items", "Trade", "Portfolio", "Govern", "Settings", "Contact"].map(
          (text, index) => (
            <ListItem key={text} disablePadding onClick={() => setSelected(text)}>
              <ListItemButton
                selected={text == selected}
                sx={{
                  mt: "22px",
                  px: "30px",
                  display: "grid",
                  gridTemplateColumns: "1fr 1.5fr",
                  justifyContent: "center",
                  borderRadius: "15px",
                  "&:hover": {
                    backgroundColor: "rgba(203, 185, 247, 0.10)",
                  },
                  "&.Mui-selected": {
                    backgroundColor: "rgba(203, 185, 247, 0.12)",
                  },
                  "&.Mui-selected:hover": {
                    backgroundColor: "rgba(203, 185, 247, 0.10)",
                  },
                }}
              >
                <Box mr={2} display="flex" justifyContent={"flex-end"}>
                  <Image
                    src={"/" + text + ".png"}
                    width={25}
                    height={22}
                    alt={"image"}
                  ></Image>
                </Box>
                <Box>
                  <ListItemText primary={text} />
                </Box>
              </ListItemButton>
            </ListItem>
          )
        )}
      </List>
      <Divider />
    </Box>
  );

  const container =
    window !== undefined ? () => window().document.body : undefined;

  return (
    <Box sx={{ display: "flex" }}>
      <CssBaseline />
      <AppBar
        position="fixed"
        sx={{
          width: { md: `calc(100% - ${drawerWidth}px)` },
          ml: { md: `${drawerWidth}px` },
          pt: { xs: 1, md: 3 },
          pb: 1,
          boxShadow: "none",
        }}
      >
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open drawer"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{ mr: 2, display: { md: "none" } }}
          >
            <MenuIcon />
          </IconButton>
          <NavBar wallet={wallet}/>
        </Toolbar>
      </AppBar>
      <Box
        component="nav"
        sx={{ width: { md: drawerWidth }, flexShrink: { md: 0 } }}
        aria-label="mailbox folders"
      >
        {/* The implementation can be swapped with js to avoid SEO duplication of links. */}
        <Drawer
          container={container}
          variant="temporary"
          open={mobileOpen}
          onClose={handleDrawerToggle}
          ModalProps={{
            keepMounted: true, // Better open performance on mobile.
          }}
          sx={{
            display: { xs: "block", md: "none" },
            "& .MuiDrawer-paper": {
              boxSizing: "border-box",
              width: drawerWidth,
            },
          }}
        >
          {drawer}
        </Drawer>
        <Drawer
          variant="permanent"
          sx={{
            display: { xs: "none", md: "block" },
            "& .MuiDrawer-paper": {
              boxSizing: "border-box",
              width: drawerWidth,
            },
          }}
          open
        >
          {drawer}
        </Drawer>
        <Toolbar />
      </Box>
      <Box
        component="main"
        sx={{
          flexGrow: 1,
          p: 3,
          width: { md: `calc(100% - ${drawerWidth}px)` },
        }}
      >
        <Toolbar />
        {children}
      </Box>
    </Box>
  );
}

ResponsiveDrawer.propTypes = {
  /**
   * Injected by the documentation to work in an iframe.
   * You won't need it on your project.
   */
  window: PropTypes.func,
};

export default ResponsiveDrawer;
