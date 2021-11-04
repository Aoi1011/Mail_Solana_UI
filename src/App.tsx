import React from "react";
import { BrowserRouter as Router, Switch, Route } from "react-router-dom";
import { CssBaseline, ThemeProvider } from "@mui/material";
import theme from "./theme";
// import Navbar from "./components/Navbar";
import Signin from "./pages/loggedout/Signin";
import Main from "./pages/loggedin";
import { store } from "./store";
import { Provider } from "react-redux";

function App() {
  return (
    <Provider store={store}>
      <Router>
        <ThemeProvider theme={theme}>
          <CssBaseline />
          <Switch>
            <Route path="/mail" exact component={Main} />
            <Route path="/" exact component={Signin} />
          </Switch>
        </ThemeProvider>
      </Router>
    </Provider>
  );
}

export default App;
