import * as React from "react";
import PropTypes from "prop-types";
import { Alert, Collapse, IconButton } from "@mui/material";
import { Close } from "@mui/icons-material";
import Typography from "@mui/material/Typography";

export function AlertSuccess(props) {
  const [collapse, setCollapse] = React.useState(true);

  const wasCalled = React.useRef(false);
  React.useEffect(() => {
    if (wasCalled.current) return;
    wasCalled.current = true;
    if (props.onClear) props.onClear();
  }, [props]);

  return (
    <Collapse in={collapse}>
      <Alert
        style={props.style}
        severity="success"
        action={
          <IconButton
            aria-label="close"
            color="white.main"
            size="small"
            onClick={() => {
              setCollapse(false);
              if (props.onClose) {
                setTimeout(() => props.onClose(), 200);
              }
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

AlertSuccess.propTypes = {
  style: PropTypes.object,
  onClose: PropTypes.func,
  onClear: PropTypes.func,
  children: PropTypes.string.isRequired,
};
