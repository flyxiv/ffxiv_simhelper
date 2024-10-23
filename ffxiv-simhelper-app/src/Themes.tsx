enum ServiceMode {
  App = 'application',
  Server = 'server',
}

export const AppConfigurations = {
  backgroundOne: "#121212",
  backgroundTwo: "#201F28",
  backgroundThree: "#2F2D3C",
  backgroundFour: "#514C76",
  backgroundButton: "#BB86FC",

  primary: "#BB86FC",
  primaryVariant: "#3700B3",
  secondary: "#03DAC6",
  secondaryVariant: "#018786",

  alert: "#CF6679",

  body1FontSize: {
    xs: 12,
    sm: 15,
    md: 18,
    lg: 14,
    xl: 18
  },
  body2FontSize: {
    xs: 8,
    sm: 11,
    md: 14,
    lg: 10,
    xl: 14
  },
  logoFontSize: {
    xs: 12,
    sm: 12,
    md: 12,
    lg: 12,
    xl: 12
  },


  isApp: process.env.NODE_ENV !== ServiceMode.Server,
  requestServer: process.env.NODE_ENV !== ServiceMode.Server ? "http://localhost:13406" : "https://www.ffxivsimhelper.com:13406"
};

