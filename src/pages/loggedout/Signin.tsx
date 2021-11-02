import React from "react";
import { Box, Stack, TextField, Typography } from "@mui/material";
import { LoadingButton } from "@mui/lab";

const Signin = (props: any) => {
  const handleSignin = async () => {
    props.history.push("/mail/inbox");
  };

  return (
    <Box
      sx={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        height: "100vh",
      }}
    >
      <Stack
        direction="column"
        justifyContent="center"
        alignItems="center"
        spacing={1}
      >
        <Typography variant="h5">Welcome to SolMail</Typography>
        <Typography variant="caption">Connect your wallet to signin</Typography>
        <TextField id="account-seed" label="Account Seed" required />
        <LoadingButton
          variant="contained"
          size="medium"
          color="secondary"
          onClick={handleSignin}
        >
          Signin
        </LoadingButton>
      </Stack>
    </Box>
  );
};

export default Signin;
