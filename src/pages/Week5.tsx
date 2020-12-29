import React from "react";

const Week5: React.FC = () => {
    const [hashes, setHashes] = React.useState("");
    const [bchHashes, setBchHashes] = React.useState("");
    const [passwords, setPasswords] = React.useState<string[]>([]);
    const [bchCodes, setBchCodes] = React.useState<string[]>([]);

    const ref = React.useRef<HTMLFormElement>(null);

    const handleInputChange = (e: any) => {
        setHashes(e.target.value);
    }

    const handleBCHChange = (e: any) => {
        setBchHashes(e.target.value);
    }

    const submitForCracking = (e: any) => {
        e.preventDefault();
        let body = hashes.split('\n');
        setPasswords([]);
        fetch(`http://127.0.0.1:8080/crack/`, {body: JSON.stringify(body), method: "POST"}).then((response) => {
            response.json().then((json) => {
                setPasswords(json);
            });
        });
    }

    const submitForBchCracking = (e: any) => {
        e.preventDefault();
        let body = bchHashes.split('\n');
        setBchCodes([]);
        fetch(`http://127.0.0.1:8080/crackbch/`, {body: JSON.stringify(body), method: "POST"}).then((response) => {
            response.json().then((json) => {
                setBchCodes(json);
            });
        });
    }

    const clear = (e: any) => {
        e.preventDefault();
        setHashes("");
        setPasswords([]);

        ref.current?.reset();
    }

    const clearBch = (e: any) => {
        e.preventDefault();
        setBchHashes("");
        setBchCodes([]);
    }

    let password_output_lines = [];
    if (passwords.length !== 0) {
        let hash_list = hashes.split('\n');
        for (let i = 0; i < passwords.length; i++) {
            password_output_lines.push(
                <tr key={i}>
                    <td>{hash_list[i]}</td>
                    <td>{(passwords[i].length > 0) ? passwords[i] : "Not Found"}</td>
                </tr>);
        }
    }

    let bch_output_lines = [];
    if (bchCodes.length !== 0) {
        let hash_list = bchHashes.split('\n');
        for (let i = 0; i < bchCodes.length; i++) {
            bch_output_lines.push(
                <tr key={i}>
                    <td>{hash_list[i]}</td>
                    <td>{(bchCodes[i].length > 0) ? bchCodes[i] : "Not Found"}</td>
                </tr>);
        }
    }

    return (
        <div>
            <h1>Password Decryption</h1>
            <p>Use brute force to decrypt a SHA1 password</p>
            <div style={{display: "flex", flexWrap: "wrap"}}>
                <div style={{margin: "0 10px"}}>
                    <form onSubmit={submitForCracking}>
                        <label>
                            <textarea style={{width: "34rem"}} className={"form-field animation"} required={true} onChange={handleInputChange} 
                            placeholder={"Password hashes"} rows={20} value={hashes}/>
                        </label>
                        <br/>
                        <br/>
                        <button className={"form-button"}>Crack hash</button>
                        <button type={"reset"} className={"form-button"} onClick={clear}>Clear</button>
                    </form>
                </div>
                <div>
                    <form onSubmit={submitForBchCracking}>
                        <label>
                            <textarea style={{width: "34rem"}} className={"form-field animation"} required={true} onChange={handleBCHChange}
                            placeholder={"BCH Hashes"} rows={20} value={bchHashes}/>
                        </label>
                        <br/>
                        <br/>
                        <button className={"form-button"}>Crack BCH Hash</button>
                        <button className={"form-button"} onClick={clearBch}>Clear</button>
                    </form>
                </div>
            </div>
            <div style={{display: "flex", justifyItems: "center", marginTop: "5rem"}}>
                {password_output_lines.length !== 0 ? (
                    <table>
                        <tr>
                            <th>Hash</th>
                            <th>Password</th>
                        </tr>
                        {password_output_lines}
                    </table>
                ) : ""}
            </div>
            <div style={{display: "flex", justifyItems: "center", marginTop: "5rem"}}>
                {bch_output_lines.length !== 0 ? (
                    <table>
                        <tr>
                            <th>Hash</th>
                            <th>BCH Input</th>
                        </tr>
                        {bch_output_lines}
                    </table>
                ) : ""}
            </div>
        </div>
    );
}

export default Week5;
