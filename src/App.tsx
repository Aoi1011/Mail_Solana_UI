import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { CssBaseline, ThemeProvider } from "@mui/material";
import theme from "./theme";
import Navbar from "./components/Navbar";
import Signin from "./pages/loggedout/Signin";
import Main from "./pages/loggedin";

function App() {
  return (
    <Router>
      <ThemeProvider theme={theme}>
        <CssBaseline />
        <Switch>
          {/* <Route path="/" component={Navbar} /> */}
          <Route path="/mail" exact component={Main} />
          <Route path="/" exact component={Signin} />
        </Switch>
      </ThemeProvider>
    </Router>
  );
}

export default App;
