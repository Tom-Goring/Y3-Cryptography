import React from "react";

const Week4: React.FC = () => {
    const [input, setInput] = React.useState("");
    const [hash, setHash] = React.useState("");

    const ref = React.useRef<HTMLFormElement>(null);

    const handleInputChange = (e: any) => {
        setInput(e.target.value);
    }

    const submitForHashing = (e: any) => {
        e.preventDefault();
        fetch(`http://127.0.0.1:8080/hash/${input}`).then((response) => {
            response.text().then((text) => {
                setHash(text);
            });
        });
    }

    const clear = (e: any) => {
        e.preventDefault();
        setInput("");
        setHash("");
        ref.current?.reset();
    }

    return (
        <div>
            <h1>Password Encryption</h1>
            <p>Encrypt a given password using SHA1</p>

            <form onSubmit={submitForHashing}>
                <label>
                    <input className={"form-field animation"} required={true} onChange={handleInputChange}
                        placeholder={"Input to hash"} value={input}/>
                </label>
                <button className={"form-button"}>Hash input</button>
                <button type={"reset"} className={"form-button"} onClick={clear}>Clear</button>
            </form>
            <p>{hash}</p>
        </div>
    );
}

export default Week4;
