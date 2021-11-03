import React from "react";
import {
  Avatar,
  Box,
  Button,
  List,
  ListItem,
  ListItemAvatar,
  ListItemText,
  Paper,
  Typography,
  useTheme,
} from "@mui/material";
import {
  Reply as ReplyIcon,
  ForwardToInbox as ForwardToInboxIcon,
  People as PeopleIcon,
} from "@mui/icons-material";

const ViewMail = (props: any) => {
  const theme = useTheme();

  return (
    <Box
      sx={{
        display: "flex",
        height: { sm: "calc(100vh - 64px)", xs: "calc(100vh - 56px)" },
        "& .MuiTypography-root": { m: 1, width: "90%" },
      }}
      //   autoComplete='off'
    >
      <Paper
        sx={{
          display: "flex",
          flexDirection: "column",
          alignItems: "flex-start",
          pt: theme.spacing(1),
          width: "100%",
        }}
        elevation={3}
      >
        <Typography variant="h3">subject</Typography>

        <List sx={{ width: "100%", p: 0 }}>
          <ListItem sx={{ p: 0, pl: 1 }}>
            <ListItemAvatar sx={{ minWidth: 40 }}>
              <Avatar>
                <PeopleIcon />
              </Avatar>
            </ListItemAvatar>
            <ListItemText
              primary={
                <Box
                  sx={{
                    display: "flex",
                    alignItems: "center",
                  }}
                >
                  <Typography variant="subtitle2">From Address</Typography>
                  <Typography variant="caption">
                    {new Date().toLocaleDateString()}
                  </Typography>
                </Box>
              }
              secondary={<Typography variant="caption">to Address</Typography>}
            />
          </ListItem>
        </List>

        <Typography sx={{ textAlign: "justify", overflowWrap: "break-word" }}>
          body
        </Typography>

        <Box
          sx={{
            display: "flex",
            width: "100%",
            justifyContent: "space-around",
            mb: 1,
          }}
        >
          <Button
            color="secondary"
            variant="outlined"
            type="submit"
            startIcon={<ReplyIcon />}
          >
            Reply
          </Button>
          <Button
            color="secondary"
            variant="outlined"
            startIcon={<ForwardToInboxIcon />}
          >
            Forward
          </Button>
        </Box>
      </Paper>
    </Box>
  );
};

export default ViewMail;
