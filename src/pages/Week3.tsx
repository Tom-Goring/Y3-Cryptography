import React from "react";

const Week3: React.FC = () => {
    const [bch, setBch] = React.useState("");
    const [result, setResult] = React.useState("");

    const ref = React.useRef<HTMLFormElement>(null);
    
    const handleInputChange = (e: any) => {
        setBch(e.target.value);
    }

    const submitForVerification = (e: any) => {
        e.preventDefault();
        fetch(`http://127.0.0.1:8080/bch/${bch}`).then((response) => {
            response.text().then((text) => {
                setResult(text);
            });
        });
    }

    const clear = (e: any) => {
        e.preventDefault();
        setBch("");
        setResult("");
        ref.current?.reset();
    }

    return (
        <div>
            <h1>BCH Codes</h1>
            <p>Read a BCH (10,6) code and verify its integrity, printing any errors and a correction if invalid.</p>
            <form onSubmit={submitForVerification} ref={ref}>
                <label>
                    <input className={"form-field animation"} type={"tel"} minLength={10} maxLength={10} 
                    required={true} onChange={handleInputChange} placeholder={"Enter a ten digit number"}
                    value={bch}/>
                </label>
                <button className={"form-button"}>Verify BCH Code</button>
                <button type={"reset"} className={"form-button"} onClick={clear}>Clear</button>
            </form>
            <p>{result}</p>
        </div>
    );
}

export default Week3;
