import React from "react";
// @ts-ignore
import {Link} from "react-router-dom";
import { RawRoute } from '../App';

interface NavbarProps {
    routes: RawRoute[],
    active: boolean
}

const Navbar: React.FC<NavbarProps> = (props: NavbarProps) => {
    let nav_items = [];
    let counter = 0;

    console.log(props.active);

    for (let route of props.routes) {
        counter++;
        if (route.sub_routes === undefined) {
            nav_items.push(
                <li className="chapter-item">
                    <Link to={route.path}>
                        <strong>{counter}. </strong>
                        {route.label}
                    </Link>
                </li>
            )
        } else {
            let interior_items = [];
            let sub_items = 0;

            nav_items.push(
                <li className="chapter-item expanded">
                    <Link to={route.path}>
                        <strong>{counter}. </strong>
                        {route.label}
                    </Link>
                </li>
            )

            for (let sub_route of route.sub_routes) {
                sub_items++;
                interior_items.push(
                    <li className="chapter-item expanded">
                        <Link to={sub_route.path}>
                            <strong>{counter}.{sub_items}. </strong>
                            {sub_route.label}
                        </Link>
                    </li>
                )
            }
            nav_items.push(
                <li>
                    <ol className="section expanded">
                        {interior_items}
                    </ol>
                </li>
            )
        }
    }

    return (
        <div className={`${props.active ? "sidebar-visible" : "sidebar-hidden"}`}>
            <nav className="sidebar">
                <div className="sidebar-scrollbox">
                    <ol className="chapter">
                        {nav_items}
                    </ol>
                </div>
            </nav>
        </div>
    );
}

export default Navbar;
