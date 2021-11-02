import React, { useState } from "react";
import {
  AppBar,
  Box,
  Divider,
  Drawer,
  IconButton,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Toolbar,
} from "@mui/material";
import {
  Inbox as InboxIcon,
  Logout as LogoutIcon,
  Mail as MailIcon,
  Menu as MenuIcon,
} from "@mui/icons-material";

const drawerWidth = 240;
const menuItems = [
  {
    link: "/mail/inbox",
    name: "Inbox",
    icon: <InboxIcon />,
  },
  {
    link: "/mail/sent",
    name: "Sent",
    icon: <MailIcon />,
  },
];

const Navbar = (props: any) => {
  const [mobileOpen, setMobileOpen] = useState(false);

  const menu = (
    <div>
      <Toolbar
        sx={{
          display: "flex",
          justifyContent: "center",
          height: { sm: 64, xs: 48 },
        }}
      >
        <img
          src={`${process.env.PUBLIC_URL}/sol-icon.png`}
          style={{ maxHeight: "70%" }}
          alt="logo"
        />
      </Toolbar>
      <Divider />
      <List>
        {menuItems.map((el, index) => (
          <ListItemButton>
            <ListItemIcon>{el.icon}</ListItemIcon>
            <ListItemText primary={el.name} />
          </ListItemButton>
        ))}
        <Divider />
        <ListItemButton>
          <ListItemIcon>
            <LogoutIcon />
          </ListItemIcon>
          <ListItemText primary="Logout" />
        </ListItemButton>
      </List>
    </div>
  );

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen);
  };

  const container =
    props.window !== undefined ? () => props.widow().document.body : undefined;

  return (
    <Box sx={{ display: "flex" }}>
      <AppBar
        position="fixed"
        sx={{
          width: { sm: `calc(100% - ${drawerWidth}px)` },
          ml: { sm: `${drawerWidth}` },
        }}
      >
        <Toolbar>
          <IconButton
            color="inherit"
            aria-label="open-drawer"
            edge="start"
            onClick={handleDrawerToggle}
            sx={{ mr: 2, display: { sm: "none" } }}
          >
            <MenuIcon />
          </IconButton>
        </Toolbar>
      </AppBar>
      <Box
        component="nav"
        sx={{ width: { sm: drawerWidth }, flexShrink: { sm: 0 } }}
        aria-label="mailbox folders"
      >
        <Drawer
          container={container}
          variant="temporary"
          open={mobileOpen}
          onClose={handleDrawerToggle}
          ModalProps={{ keepMounted: true }}
          sx={{
            display: { xs: "block", sm: "none" },
            "& .MuiDrawer-paper": {
              boxSizing: "border-box",
              width: drawerWidth,
            },
          }}
        >
          {menu}
        </Drawer>
        <Drawer
          variant="permanent"
          sx={{
            display: { xs: "none", sm: "block" },
            "& .MultiDrawer-paper": {
              boxSizing: "border-box",
              width: drawerWidth,
            },
          }}
          open
        >
          {menu}
        </Drawer>
      </Box>
    </Box>
  );
};

export default Navbar;
