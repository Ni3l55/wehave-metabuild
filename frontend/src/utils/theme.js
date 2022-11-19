import { createTheme, responsiveFontSizes } from "@mui/material/styles";
import { red } from "@mui/material/colors";

// Create a theme instance.
const theme = responsiveFontSizes(
  createTheme({
    palette: {
      primary: {
        main: "#020710",
        contrastText: "#fff",
      },
      secondary: {
        main: "#0B1224",
        contrastText: "#fff",
      },
      tertiary: {
        main: "#7785a8",
        contrastText: "#fff",
      },
      error: {
        main: red.A400,
      },
      text: {
        primary: "#FFFF",
        secondary: "#BBA0FE",
        card: "#FFFFFF80",
        price: "#C3C3C3",
        profit: "#00E3A4",
        loss: "#F43030",
      },
      background: {
        default: "#020710",
        secondary: "#0B101A",
        tertiary: "#465e9c"
      },
    },
    shape: {
      borderRadius: 15,
    },
    typography: {
      allVariants: {
        fontFamily: "Poppins",
        color: "#FFFFFF",
        lineHeight: "1.23",
      },
      body1: {
        fontSize: "20px",
        fontWeight: "400",
      },
      body2: {
        fontSize: "12px",
        fontWeight: "400",
        lineHeight: 1.4,
      },
      body3: {
        fontSize: "16px",
        fontWeight: "700",
      },
      h1: {
        fontFamily: "Montserrat",
        fontSize: "24px",
        lineHeight: "1.23",
        fontWeight: "600",
      },
      h2: {
        fontFamily: "Poppins",
        fontSize: "20px",
        lineHeight: "1.23",
        fontWeight: "700",
      },
      h3: {
        fontFamily: "Montserrat",
        fontSize: "20px",
        fontWeight: "400",
      },
      h4: {
        fontFamily: "Poppins",
        fontSize: "24px",
        fontWeight: "700",
      },
      caption: {
        fontSize: "16px",
      },
    },
    components: {
      MuiButtonGroup: {
        styleOverrides: {
          root: {
            borderRadius: 20,
          },
        },
      },
      MuiButton: {
        variants: [
          {
            props: { variant: "group" },
            style: { padding: "17px" },
          },
        ],
        defaultProps: {
          disableRipple: true,
        },

        styleOverrides: {
          sizeLarge: {
            padding: "20px 50px",
          },
          contained: {
            borderRadius: "12px",
            background:
              "linear-gradient(94.05deg, #B9B7DB 0%, rgba(113, 197, 242, 0.96) 100%)",
            fontSize: "11px",
            lineHeight: "16px",
            fontFamily: "'Poppins'",
            fontWeight: 500,
            padding: "15px 40px",
            textTransform: "none",
          },
          containedSmall: {
            borderRadius: "12px",
            background: "#BBA0FE",
            fontSize: "12px",
            lineHeight: "16px",
            fontFamily: "'Poppins'",
            fontWeight: 500,
            padding: "5px 15px",
            textTransform: "none"
          },
          containedApproved: {
            borderRadius: "12px",
            background: "green",
            fontSize: "11px",
            lineHeight: "16px",
            fontFamily: "'Poppins'",
            fontWeight: 500,
            padding: "15px 40px",
            textTransform: "none",
          },
          text: {
            background: "transparent",
            fontSize: "15px",
            lineHeight: "6px",
            fontFamily: "'Poppins'",
            fontWeight: 700,
            paddingLeft: "0px",
            textTransform: "none",
          },
          trade: {
            background: "transparent",
            fontSize: "12px",
            lineHeight: "6px",
            fontFamily: "Montserrat",
            fontWeight: 600,
            paddingLeft: "0px",
            textTransform: "none",
            color:
              "linear-gradient(91.76deg, #BBA0FE 0%, rgba(113, 197, 242, 0.96) 100%)",
          },
          tradeBorder: {
            borderRadius: "10px",
            background:
              "linear-gradient(90deg, rgba(195,142,215,1) 0%, rgba(148,187,233,1) 100%);",
            fontSize: "16px",
            lineHeight: "6px",
            fontFamily: "Montserrat",
            fontWeight: 600,
            padding: "15px 15px",
            textTransform: "none",
            color:
              "linear-gradient(91.76deg, #BBA0FE 0%, rgba(113, 197, 242, 0.96) 100%)",
          },
          tradeBorderNoBg: {
            borderRadius: "10px",
            border: '1px solid',
            borderImageSlice: 1,
            borderImageSource: `linear-gradient(91.76deg, #BBA0FE 0%, rgba(113, 197, 242, 0.96) 100%)`,
            background: "transparent",
            fontSize: "16px",
            lineHeight: "6px",
            fontFamily: "Montserrat",
            fontWeight: 600,
            padding: "15px 15px",
            textTransform: "none",
            color:
              "linear-gradient(91.76deg, #BBA0FE 0%, rgba(113, 197, 242, 0.96) 100%)",
          }
        },
      },
    },
  })
);

export default theme;
