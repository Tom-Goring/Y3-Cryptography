import React from "react";

const Week2: React.FC = () => {
    const [sixDigitInput, setSixDigitInput] = React.useState("");
    const [tenDigitInput, setTenDigitInput] = React.useState("");
    const [checkDigits, setCheckDigits] = React.useState("");
    const [syndrome, setSyndrome] = React.useState("");

    const sixRef = React.useRef<HTMLFormElement>(null);
    const tenRef = React.useRef<HTMLFormElement>(null);

    const handleSixDigitChange = (e: any) => {
        setSixDigitInput(e.target.value);
    }

    const submitSixDigit = (e: any) => {
        e.preventDefault();
        console.log(sixDigitInput)
        fetch(`http://127.0.0.1:8080/hamming/checkdigits/${sixDigitInput}`).then((response) => {
            response.text().then((text) => {
                setCheckDigits(text);
            });
        });
    }

    const clearSixDigitInput = (e: any) => {
        e.preventDefault();
        setSixDigitInput("");
        setCheckDigits("");
        sixRef.current?.reset();
    }

    const handleTenDigitChange = (e: any) => {
        setTenDigitInput(e.target.value);
    }

    const submitTenDigit = (e: any) => {
        e.preventDefault();
        console.log(tenDigitInput)
        fetch(`http://127.0.0.1:8080/hamming/syndromes/${tenDigitInput}`).then((response) => {
            response.text().then((text) => {
                setSyndrome(text);
            });
        });
    }

    const clearTenDigitInput = (e: any) => {
        e.preventDefault();
        setTenDigitInput("");
        setSyndrome("");
        tenRef.current?.reset();
    }

    return (
        <div>
            <h1>Hamming Codes</h1>
            <p>Take a 6 digit input and generate 4 extra digits.</p>
            <p>Take a 10 digit input and generate 4 syndrome digits.</p>
            <form ref={sixRef} onSubmit={submitSixDigit}>
                <h2>Six Digit Input</h2>
                <label>
                    <input className={"form-field animation"} type={"tel"}
                           minLength={6} maxLength={6} required={true} value={sixDigitInput}
                           onChange={handleSixDigitChange} placeholder={"Enter a six digit number"}/>
                </label>
                <button className={"form-button"}>Generate check digits</button>
                <button className={"form-button"} onClick={clearSixDigitInput}>Clear</button>
            </form>
            <p>{checkDigits}</p>
            <form ref={tenRef} onSubmit={submitTenDigit}>
                <h2>Ten Digit Input</h2>
                <label>
                    <input className={"form-field animation"} type={"tel"}
                           minLength={10} maxLength={10} required={true} value={tenDigitInput}
                           onChange={handleTenDigitChange} placeholder={"Enter a ten digit number"}/>
                </label>
                <button className={"form-button"}>Verify syndrome digits</button>
                <button className={"form-button"} onClick={clearTenDigitInput}>Clear</button>
            </form>
            <p>{syndrome}</p>
        </div>
    );
}

export default Week2;
