import React, { useState } from "react";
import { useDispatch, useSelector } from "react-redux";
import { connectWallet } from "../../store/actions/accountActions";
import { Box, Stack, TextField, Typography } from "@mui/material";
import { LoadingButton } from "@mui/lab";

const Signin = (props: any) => {
  const [seed, setSeed] = useState("");

  const dispatch = useDispatch();
  const loading = useSelector((state: any) => state.account.loading);

  const handleSignin = async () => {
    await dispatch(connectWallet(seed));
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
        <TextField
          id="account-seed"
          label="Account Seed"
          value={seed}
          onChange={(event) => setSeed(event.target.value)}
          required
        />
        <LoadingButton
          variant="contained"
          size="medium"
          color="secondary"
          onClick={handleSignin}
          loading={loading}
        >
          Signin
        </LoadingButton>
      </Stack>
    </Box>
  );
};

export default Signin;
