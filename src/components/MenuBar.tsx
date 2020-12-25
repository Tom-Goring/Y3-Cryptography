import React from "react";

import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faBars } from '@fortawesome/free-solid-svg-icons'

interface MenuBarProps {
    setSideBarOpen: Function
}

const MenuBar: React.FC<MenuBarProps> = (props: MenuBarProps) => {
    const scrollRef = React.useRef({
        lastScrollWasUpward: false,
        previousHeight: window.scrollY,
    });

    const ref = React.useRef<HTMLDivElement>(null);

    React.useEffect(() => {
        const handleScroll = () => {
            let downwards = window.scrollY > scrollRef.current.previousHeight;
            scrollRef.current.previousHeight = window.scrollY;

            if (!downwards) {
                if (ref.current) {
                    if (!scrollRef.current.lastScrollWasUpward) {
                        ref.current.style.top = (window.scrollY - ref.current.clientHeight - 50) + 'px';
                    } else {
                        if (window.scrollY < parseFloat(ref.current.style.top.slice(0, -2))) {
                            ref.current.classList.add("sticky")
                        }
                    }
                }
            } else {
                if (ref.current) {
                    if (scrollRef.current.lastScrollWasUpward) {
                        ref.current.style.top = window.scrollY + 'px';
                    }
                    ref.current.classList.remove("sticky");
                }
            }

            if (window.scrollY === 0) {
                ref.current?.classList.add("sticky");
            }

            scrollRef.current.lastScrollWasUpward = !downwards;
        }

        const handleBorder = () => {
            if (ref.current!.offsetTop < 3) {
                ref.current?.classList.remove("bordered");
            } else {
                ref.current?.classList.add("bordered");
            }
        }

        window.addEventListener('scroll', handleScroll, {passive: true});
        window.addEventListener('scroll', handleBorder, {passive: true});

        return () => {
            window.removeEventListener('scroll', handleScroll);
            window.removeEventListener('scroll', handleBorder);
        }

    }, []);

    const toggleSidebar = () => {
        console.log("Toggling sidebar");
        props.setSideBarOpen((state: boolean) => !state);
    }

    return (
            <div id={"menu-bar"}
                 className={"menu-bar"}
                 ref={ref}
            >
                <div className={"left-buttons"}>
                    <button onClick={toggleSidebar} className={"icon-button"} id={"sidebar-toggle"}>
                        <FontAwesomeIcon icon={faBars}/>
                    </button>
                </div>
                <h1 className={"menu-title"}>Cryptography</h1>
                <div className={"right-buttons"}/>
            </div>
    );
}

export default MenuBar;
