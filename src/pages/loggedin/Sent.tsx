import React, { Fragment } from "react";
import { Fab } from "@mui/material";
import { Edit as EditIcon } from "@mui/icons-material";
import MailTable from "../../components/MailTable";

const Sent = (props: any) => {
  return (
    <Fragment>
      <MailTable data={[]} history={props.history} />
      <Fab
        variant="extended"
        size="large"
        color="secondary"
        aria-label="compose mail"
        onClick={() => props.history.push("send")}
        sx={{
          position: "absolute",
          bottom: 48,
          right: 16,
        }}
      >
        <EditIcon sx={{ mr: 1 }} />
        Send Mail
      </Fab>
    </Fragment>
  );
};

export default Sent;
