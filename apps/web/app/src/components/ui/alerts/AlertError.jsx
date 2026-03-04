import * as React from "react";
import PropTypes from "prop-types";
import { Alert, Collapse, IconButton } from "@mui/material";
import { Close } from "@mui/icons-material";
import Typography from "@mui/material/Typography";

export function AlertError(props) {
  const { onClose } = props;

  const [collapse, setCollapse] = React.useState(true);

  return (
    <Collapse in={collapse}>
      <Alert
        style={props.style}
        severity="error"
        action={
          <IconButton
            aria-label="close"
            color="white.main"
            size="small"
            onClick={() => {
              setCollapse(false);
              setTimeout(() => onClose(), 200);
            }}
          >
            <Close color="white" fontSize="inherit" />
          </IconButton>
        }
      >
        <Typography gutterBottom variant="text2">
          {props.children}
        </Typography>
      </Alert>
    </Collapse>
  );
}

AlertError.propTypes = {
  style: PropTypes.object,
  onClose: PropTypes.func.isRequired,
  children: PropTypes.string.isRequired,
};
