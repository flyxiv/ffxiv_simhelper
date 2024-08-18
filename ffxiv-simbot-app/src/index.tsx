import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";
import { ThemeProvider, createTheme } from "@mui/material";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

const theme = createTheme({
  palette: {
    primary: {
      main: "#BB86FC",
    },

    secondary: {
      main: "#03DAC6",
    },
  },
  typography: {
    fontFamily: "Arial",
    h1: {
      fontSize: "3rem",
      fontWeight: 900,
    },
    h2: {
      fontSize: "1.75rem",
      fontWeight: 600,
    },
    h3: {
      fontSize: "1.5rem",
      fontWeight: 600,
    },
  },

  components: {
    MuiOutlinedInput: {
      defaultProps: {
        notched: false,
      },
    },
    MuiInputLabel: {
      defaultProps: {
        shrink: false,
      },
    },
  },
});

root.render(
  <React.StrictMode>
    <ThemeProvider theme={theme}>
      <App />
    </ThemeProvider>
  </React.StrictMode>
);

