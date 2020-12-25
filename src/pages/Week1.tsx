import React from "react";
import Cleave from "cleave.js/react";

interface CCNProps {
    setCCN: Function
}

const CreditInput: React.FC<CCNProps> = (props: CCNProps) => {
    return (
        <Cleave
            className={"form-field animation"}
            placeholder="Enter CCN"
            options={{blocks: [4, 4, 4, 4]}}
            onChange={(event) => props.setCCN(event.target.rawValue)}
            style={{textOverflow: "ellipsis"}}
            minLength={16}
            maxLength={16}
            required={true}
        />
    )
}

const Week1: React.FC = () => {
    const [ISBN, setISBN] = React.useState("");
    const [CCN, setCCN] = React.useState("");

    const onISBNChange = (e: any) => {
        const re = /^[0-9\b]+$/;

        if (e.target.value === '' || re.test(e.target.value)) {
            setISBN(e.target.value);
        }
    }

    return (
        <div className={"content"}>
            <h3>Credit and ISBN Verification</h3>
            <p>This week's task is to verify given ISBN and credit card numbers.</p>
            <form onSubmit={ (e) => {
                e.preventDefault();
                console.log(ISBN);
            }}>
                <h2>ISBN Code</h2>
                <label>
                    <input placeholder={"Enter ISBN"} className={"form-field animation"} type={"tel"} inputMode={"numeric"} value={ISBN} onChange={onISBNChange} minLength={10} maxLength={10} required={true}/>
                </label>
                <button>Verify ISBN</button>
            </form>
            <form onSubmit={(e) => {
               e.preventDefault();
               console.log(CCN);
            }}>
                <label>
                    <h2>Credit Card Number</h2>
                    <CreditInput setCCN={setCCN}/>
                </label>
                <button>Verify CCN</button>
            </form>
        </div>
    );
}

export default Week1;
