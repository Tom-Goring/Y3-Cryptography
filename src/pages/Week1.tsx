import React from "react";

const Week1: React.FC = () => {
    const [ISBN, setISBN] = React.useState("");
    const [ISBNMessage, setISBNMessage] = React.useState("");
    const [CCNMessage, setCCNMessage] = React.useState("");
    const [CCN, setCCN] = React.useState("");

    const onISBNChange = (e: any) => {
        setISBN(e.target.value);
    }

    const submitISBN = (e: any) => {
        e.preventDefault();
        fetch(`http://127.0.0.1:8080/isbn/${ISBN}`).then((response) => {
            response.text().then((text) => {
                setISBNMessage(text);
            });
        });
    }

    const onCCNChange = (e: any) => {
        setCCN(e.target.value);
    }

    const submitCCN = (e: any) => {
        e.preventDefault();
        console.log(CCN);
        fetch(`http://127.0.0.1:8080/ccn/${CCN}`).then((response) => {
            response.text().then((text) => {
                setCCNMessage(text);
            });
        });
    }

    const clearISBN = (e: any) => {
        e.preventDefault();
        setISBN("");
        setISBNMessage("");
    }

    const clearCCN = (e: any) => {
        e.preventDefault();
        setCCN("");
        setCCNMessage("");
    }

    return (
        <div>
            <h1>Credit and ISBN Verification</h1>
            <p>This week's task is to verify given ISBN and credit card numbers.</p>
            <form onSubmit={submitISBN}>
                <h2>ISBN Code</h2>
                <label>
                    <input placeholder={"Enter ISBN"} className={"form-field animation"} type={"tel"}
                           inputMode={"numeric"} value={ISBN} onChange={onISBNChange} required={true}/>
                </label>
                <button className={"form-button"}>Verify ISBN</button>
                <button className={"form-button"} onClick={clearISBN}>Clear</button>
            </form>
            <p>{ISBNMessage}</p>
            <form onSubmit={submitCCN}>
                <label>
                    <h2>Credit Card Number</h2>
                    <input placeholder={"Enter CCN"} className={"form-field animation"} type={"tel"}
                           inputMode={"numeric"} value={CCN} onChange={onCCNChange} required={true}/>
                </label>
                <button className={"form-button"}>Verify CCN</button>
                <button className={"form-button"} onClick={clearCCN}>Clear</button>
            </form>
            <p>{CCNMessage}</p>
        </div>
    );
}

export default Week1;
