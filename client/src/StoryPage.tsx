import { Component, h } from 'preact';
import { dataCache, TreeNodeQuery } from './main';

declare var API_HOST: string;

interface StoryResult {
	tree: number;
	nodes: TreeNodeQuery[];
}

interface Props {
	id: string;
}

interface State {
	data?: StoryResult;
	loadingID: string;
}

export default class StoryPage extends Component<Props, State> {
	public render(props: Props, state: State) {
		return <div>
			{state.data ? <div>
				<p>
					{state.data.nodes.map(node => <span key={node.id}>
						{node.text}
					</span>)}
				</p>
			</div> : <div>Loading...</div>}
		</div>;
	}

	public componentWillMount() {
		this.load(this.props.id);
	}

	public componentWillReceiveProps(props: Props) {
		if(this.state.loadingID !== props.id) {
			this.load(props.id);
		}
	}

	private load(id: string) {
		this.setState({data: undefined, loadingID: id});
		dataCache.load("story:" + id, id => {
			return fetch(API_HOST + "/nodes/" + id + "/story")
				.then(res => res.json());
		}, id)
			.then(data => {
				if(id !== this.state.loadingID) return;
				this.setState({data});
			});
	}

}
