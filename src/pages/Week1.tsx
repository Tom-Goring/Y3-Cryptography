import React from "react";

const Week1: React.FC = () => {
    const [ISBN, setISBN] = React.useState("");
    const [CCN, setCCN] = React.useState("");

    const onISBNChange = (e: any) => {
        const re = /^[0-9\b]+$/;

        if (e.target.value === '' || re.test(e.target.value)) {
            setISBN(e.target.value);
        }
    }

    const onCCNChange = (e: any) => {
        const re = /^[0-9\b]+$/;

        if (e.target.value === '' || re.test(e.target.value)) {
            setCCN(e.target.value);
        }
    }

    return (
        <div className={"content"}>
            <h3>Credit and ISBN Verification</h3>
            <p>This week's task is to verify given ISBN and credit card numbers.</p>
            <form>
                <h2>ISBN Code</h2>
                <label>
                    <input className={"form-field animation"} type={"tel"} inputMode={"numeric"} value={ISBN} onChange={onISBNChange} minLength={10} maxLength={10}/>
                </label>
                <button>Verify ISBN</button>
            </form>
            <form>
                <label>
                    <h2>Credit Card Number</h2>
                    <input type={"tel"} inputMode={"numeric"} value={CCN} onChange={onCCNChange} minLength={16} maxLength={16}/>
                </label>
                <button>Verify CCN</button>
            </form>
        </div>
    );
}

export default Week1;
