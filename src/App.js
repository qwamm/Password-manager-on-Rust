import React, { Component } from "react";
//import async from "async"
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
        //alert('Отправленный ключ: ' + this.state.value);
        const postData = () => {
            console.log("This is our data", this.state.value);
            const url = "http://192.168.56.1:8080";
            fetch(url,
                {
                    method: 'POST',
                    headers: {'Content-type': 'src/json'},
                    body: JSON.stringify({state: this.state.value})
                })
                .then(response => response.json())
                .then(data => {
                    console.log("Success:", data)
                })
                .catch((error) => {
                    console.error('Error:', error);
                });
        }
        postData()
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


export default function MyApp() {
    return (
        <div>
            <h1 style={{ padding: "10px 20px", textAlign: "center", color: "blue"}}>Password manager</h1>
            <ControlledFormComponent/>
        </div>
    );
}
