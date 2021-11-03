import React from "react";
import { Box, Paper, TextField, useTheme } from "@mui/material";
import { Send as SendIcon } from "@mui/icons-material";
import { LoadingButton } from "@mui/lab";

const SendMail = (props: any) => {
  const theme = useTheme();

  return (
    <Box
      component="form"
      sx={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        height: { sm: "calc(100vh - 64px)", xs: "calc(100vh - 56px)" },
        "& .MuiTextField-root": { m: theme.spacing(1), width: "90%" },
      }}
      autoComplete="off"
    >
      <Paper
        sx={{
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
          pt: theme.spacing(1),
          width: "80%",
          height: "80%",
        }}
        elevation={3}
      >
        <TextField id="to-address" label="To" required />
        <TextField id="subject" label="Subject" />
        <TextField id="body" label="Body" multiline rows={15} required />
        <Box
          sx={{
            display: "flex",
            width: "90%",
            justifyContent: "flex-end",
          }}
        >
          <LoadingButton
            color="secondary"
            variant="outlined"
            type="submit"
            startIcon={<SendIcon />}
            loadingPosition="start"
          >
            Send
          </LoadingButton>
        </Box>
      </Paper>
    </Box>
  );
};

export default SendMail;
