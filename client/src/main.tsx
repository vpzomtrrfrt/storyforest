import { Component, h, render } from 'preact';
import Router, { Route } from 'preact-router';

import 'preact/debug';

declare var API_HOST: string;

class DataCache {
	private map: {
		[key: string]: PromiseLike<any>
	} = {};

	public load<T, I>(key: string, fn: (input: I) => PromiseLike<T>, input: I) {
		if(!(key in this.map)) {
			this.map[key] = fn(input);
		}
		return this.map[key];
	}
}

const dataCache = new DataCache();

interface StoryNode {
	id: string;
	text: string;
	children: StoryNode[] | null;
}

interface NodeMap {
	[key: string]: StoryNode | undefined
};

function StoryTreeNodeLink(props: {node: StoryNode}): JSX.Element {
	return <div>
		<a href={"/nodes/" + props.node.id}>
			<div>
				{props.node.text}
			</div>
		</a>
		{props.node.children && <ul>
			{props.node.children.map(child => <li key={child.id}>
				<StoryTreeNodeLink node={child} />
			</li>)}
		</ul>}
	</div>;
}

interface NodePageProps {
	id: string
}

interface NodePageState {
	data?: StoryNode;
	loadingID: string;
}

class NodePage extends Component<NodePageProps, NodePageState> {
	public render(props: NodePageProps, state: NodePageState) {
		return <div>
			hi(node:{props.id})

			{state.data ? <div>
				<StoryTreeNodeLink node={state.data} />
			</div> : <div>Loading...</div>}
		</div>;
	}

	public componentWillMount() {
		this.load(this.props.id);
	}

	public componentWillReceiveProps(props) {		
		if(this.state.loadingID === props.id) return;
		this.load(props.id);
	}

	public load(id: string) {
		this.setState({data: undefined, loadingID: id});
		dataCache.load("node:" + id, id => {
			return fetch(API_HOST + "/nodes/" + id)
				.then(res => res.json());
		}, id)
			.then(data => {
				if(id !== this.state.loadingID) return;
				data.id = id;
				this.setState({data});
			});
	}
}

const App = function() {
	return <div>
		<Router>
			<Route path="/nodes/:id" component={NodePage} />
		</Router>
	</div>;
};

render(<App />, document.body);

console.log("aaaa");
