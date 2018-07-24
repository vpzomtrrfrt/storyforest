import { bind } from 'decko';
import { Component, h } from 'preact';
import linkState from 'linkstate';
import { route } from 'preact-router';

import { dataCache, NodeResult } from './main';

declare var API_HOST: string;

interface Props {
	parentID: string;
}

interface State {
	data?: NodeResult;
	loadingID: string;
	newText: string;
	submitting: boolean;
}

export default class NewPostPage extends Component<Props, State> {
	public render(props: Props, state: State) {
		return <div>
			{state.data ? <div>
				<p>
					<span class="parent">{state.data.text}</span>
					{state.newText}
				</p>
			<textarea value={state.newText} onInput={linkState(this, 'newText')} />
			<button disabled={state.submitting} onClick={this.submit.bind(this)}>
				Submit
			</button>
			</div> : <div>Loading...</div>}
		</div>;
	}

	public componentWillMount() {
		this.load(this.props.parentID);
	}

	public componentWillReceiveProps(props: Props) {
		if(this.state.loadingID !== props.parentID) {
			this.load(props.parentID);
		}
	}

	private load(id: string) {
		this.setState({data: undefined, loadingID: id});
		dataCache.load("node:" + id, id => {
			return fetch(API_HOST + "/nodes/" + id)
				.then(res => res.json());
		}, id)
			.then(data => {
				if(id !== this.state.loadingID) return;
				this.setState({data});
			});
	}

	private submit() {
		const parentID = this.props.parentID;
		fetch(API_HOST + "/nodes", {
			method: "post",
			headers: {
				"Content-type": "application/json"
			},
			body: JSON.stringify({
				parent: parseInt(parentID, 10),
				text: this.state.newText
			})
		})
			.then(res => res.text())
			.then(id => {
				dataCache.invalidate("node:" + parentID)
				route("/nodes/" + id);
			});
	}
}
