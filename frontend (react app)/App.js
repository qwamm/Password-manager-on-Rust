import React, { Component } from "react";
import { BrowserRouter as Router, Routes, Route} from 'react-router-dom';
class ControlledFormComponent extends Component {
    constructor(props) {
        super(props);
        this.state = {value: ''}

        this.handleChange = this.handleChange.bind(this)
        this.handleSubmit = this.handleSubmit.bind(this)
    }
    handleChange(event) {
        this.setState({value: event.target.value});
    }

    handleSubmit(event) {
        const postData = async() => {
            const url = "http://localhost:7878";
            await fetch(url,
                {
                    method: 'POST',
                    body: this.state.value
                })
                .then(response => response.text())
                .then(data => {
                    window.alert(data)
                })
                .catch((error) => {
                    console.error('Error:', error);
                });
        }
        postData();

        event.preventDefault();

    }
    render() {
        return (
            <div style={{
                display: "flex",
                justifyContent: "center",
                alignItems: "center",
                backgroundColor: "white"
            }}>
                <br/>
                <form onSubmit = {this.handleSubmit}>
                    <input type="text" placeholder="master key" value={this.state.value} onChange={this.handleChange}/>
                </form>
            </div>
        );
    }
}

function Home() {
    return (
        <div>
            <h1 style={{ padding: "10px 20px", textAlign: "center", color: "blue"}}>Password manager</h1>
            <ControlledFormComponent/>
        </div>
    );
}

function Database() {
    return (<h2>База данных</h2>);
}

export default function MyApp() {
    return (
        <div>
            <Router>
                <div>
                    <Routes>
                        <Route path='/database' element={<Database />} />
                        <Route path='/' element={<Home />} />
                    </Routes>
                </div>
            </Router>
        </div>
    );
}
