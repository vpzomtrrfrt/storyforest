import { Component, h } from 'preact';
import linkState from 'linkstate';

import { fetchStatus, showError } from './main';

declare var API_HOST: string;

interface State {
	username: string;
	password: string;
	submitting: boolean;
}

export default class LoginPage extends Component<{}, State> {
	public render(props: {}, state: State) {
		return <div>
			<form onSubmit={this.submit.bind(this)}>
				<div>
					<input type="text" name="username" placeholder="Username"  value={state.username} onChange={linkState(this, 'username')} />
				</div>
				<div>
					<input type="password" name="password" placeholder="Password" value={state.password} onChange={linkState(this, 'password')} />
				</div>
				<button type="submit" disabled={state.submitting}>Login</button>
			</form>
		</div>;
	}

	private submit(evt: Event) {
		evt.preventDefault();

		this.setState({submitting: true});
		fetch(API_HOST + "/logins", {
			method: "post",
			headers: {
				"Content-type": "application/json"
			},
			body: JSON.stringify({
				username: this.state.username,
				password: this.state.password
			})
		})
			.then(fetchStatus)
			.then(() => alert("Successfully logged in!"))
			.catch(showError)
			.then(() => this.setState({submitting: false}));
	}
}
