import React from 'react';
import './App.css';
import {
  BrowserRouter as Router,
  Switch,
  Route,
  Link,
  useLocation,
// @ts-ignore
} from "react-router-dom";

import Navbar from "./components/Navbar";
import Home from "./pages/Home";
import Week1 from "./pages/Week1";
import Week2 from "./pages/Week2";
import Week3 from "./pages/Week3";
import Week4 from "./pages/Week4";
import Week5 from "./pages/Week5";
import Week6 from "./pages/Week6";
import Week7 from "./pages/Week7";
import MenuBar from "./components/MenuBar";

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faAngleLeft, faAngleRight } from '@fortawesome/free-solid-svg-icons'

export interface RawRoute {
  path: string,
  component: React.FC<any>,
  exact: boolean,
  label: string,
  sub_routes?: RawRoute[],
}

const routes: RawRoute[] = [
  {path: "/", component: Home, exact: true, label: "Introduction"},
  {path: "/week1", component: Week1, exact: true, label: "ISBN and Credit verification"},
  {path: "/week2", component: Week2, exact: true, label: "Hamming Codes"},
  {path: "/week3", component: Week3, exact: true, label: "BCH (10,6)"},
  {path: "/week4", component: Week4, exact: true, label: "SHA1 Password Encryption"},
  {path: "/week5", component: Week5, exact: true, label: "SHA1 Password Decryption"},
  {path: "/week6", component: Week6, exact: true, label: "Ciphertexts and Steganography"},
  {path: "/week7", component: Week7, exact: true, label: "Two Time Pads"},
]

const App: React.FC = () => {
  const [sidebarOpen, setSidebarOpen] = React.useState(true);
  const location = useLocation();

  let Routes = [];
  for (let raw_route of routes) {
    if (raw_route.sub_routes !== undefined) {
      for (let sub_route of raw_route.sub_routes) {
        Routes.push(<Route path={sub_route.path} exact={sub_route.exact} component={sub_route.component}/>);
      }
    } else {
      Routes.push(<Route path={raw_route.path} exact={raw_route.exact} component={raw_route.component}/>);
    }
  }

  let currPage = routes.findIndex(route => route.path === location.pathname);

  let prevPageLink;
  let mobilePrevPageLink;
  if (currPage !== 0) {
    prevPageLink = <Link className={"nav-chapters previous"} to={routes[currPage - 1].path}><FontAwesomeIcon icon={faAngleLeft}/></Link>;
    mobilePrevPageLink = <Link className={"mobile-nav-chapters previous"} to={routes[currPage - 1].path}><FontAwesomeIcon icon={faAngleLeft}/></Link>;
  }

  let nextPageLink;
  let mobileNextPageLink;
  if (currPage !== routes.length-1) {
    nextPageLink = <Link className={"nav-chapters next"} to={routes[currPage + 1].path}><FontAwesomeIcon icon={faAngleRight}/></Link>;
    mobileNextPageLink = <Link className={"mobile-nav-chapters next"} to={routes[currPage + 1].path}><FontAwesomeIcon icon={faAngleRight}/></Link>;
  }

  return (
        <div className="App">
          <div className={`light js ${sidebarOpen ? "sidebar-visible" : "sidebar-hidden"}`}>
            <Navbar routes={routes} active={sidebarOpen}/>
            <div className={"page-wrapper"}>
              <div className={"page"}>
                <div id={"menu-bar-hover-placeholder"}/>
                <MenuBar setSideBarOpen={setSidebarOpen}/>
                <div className={"content"}>
                  <main>
                      <Switch>
                        {Routes}
                      </Switch>
                  </main>
                  <nav className={"nav-wrapper"}>
                    {mobilePrevPageLink !== undefined ? mobilePrevPageLink : ""}
                    {mobileNextPageLink !== undefined ? mobileNextPageLink : ""}
                  </nav>
                </div>
              </div>
              <nav className={"nav-wide-wrapper"}>
                {prevPageLink !== undefined ? prevPageLink : ""}
                {nextPageLink !== undefined ? nextPageLink : ""}
              </nav>
            </div>
          </div>
          </div>
  );
}

const AppWrapper: React.FC = () => {
  return(
      <Router>
        <App/>
      </Router>
  )
}

export default AppWrapper;
