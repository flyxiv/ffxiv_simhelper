import React from "react";
import ReactDOM from "react-dom/client";
import "./index.css";
import App from "./App";
import { ThemeProvider, createTheme } from "@mui/material";
import { AppConfigurations, ENGLISH_MODE } from "./Themes";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);

export const theme = createTheme({
  palette: {
    primary: {
      main: "#BB86FC",
    },

    secondary: {
      main: "#03DAC6",
    },
  },

  typography: AppConfigurations.languageMode === ENGLISH_MODE ? {
    fontFamily: "Cabinet, Oraniembaum, Oligopoly, Redwing, Arial",
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
  } : {
    fontFamily: "NotoSansKR, Arial",
    h1: {
      fontSize: "3%", fontWeight: 900,
    },
    h2: {
      fontSize: "3rem",
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

