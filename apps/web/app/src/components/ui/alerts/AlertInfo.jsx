import * as React from "react";
import PropTypes from "prop-types";
import { Alert } from "@mui/material";
import Typography from "@mui/material/Typography";

export function AlertInfo(props) {
  return (
    <Alert
      style={props.style}
      severity="info"
    >
      <Typography gutterBottom variant="text2">
        {props.children}
      </Typography>
    </Alert>
  );
}

AlertInfo.propTypes = {
  children: PropTypes.string.isRequired,
};
