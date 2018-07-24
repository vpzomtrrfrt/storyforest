import { Component, h, render } from 'preact';
import Router, { Route } from 'preact-router';

import 'preact/debug';

import './styles.scss';

import LoginPage from './LoginPage';
import NewPostPage from './NewPostPage';
import SignupPage from './SignupPage';

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

	public invalidate(key: string) {
		delete this.map[key];
	}
}

export const dataCache = new DataCache();

export function fetchStatus(response: Response) {
	if(response.status < 200 || response.status >= 300) {
		return response.text()
			.then(txt => Promise.reject(txt));
	}
	return Promise.resolve(response);
}

export function showError(err: any) {
	console.error(err);
	alert(err);
}

interface TreeNodeQuery {
	id: string;
	text: string;
}

interface StoryNode extends TreeNodeQuery {
	children: StoryNode[] | null;
}

export interface NodeResult {
	text: string;
	children: StoryNode[];
	parent: TreeNodeQuery | null;
}

interface NodeMap {
	[key: string]: StoryNode | undefined
};

function StoryTreeNodeLink(props: {node: StoryNode}): JSX.Element {
	return <div>
		<a href={"/nodes/" + props.node.id}>
			<div class="node">
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
	data?: NodeResult;
	loadingID: string;
}

class NodePage extends Component<NodePageProps, NodePageState> {
	public render(props: NodePageProps, state: NodePageState) {
		return <div>
			{state.data ? <div>
				{state.data.parent && <a href={"/nodes/" + state.data.parent.id}>
					<div class="node parent">
						{state.data.parent.text}
					</div>
				</a>}
				<div>
					<div class="node main">
						{state.data.text}
					</div>
					<ul>
						<li>
							<a href={"/postNew/" + props.id} class="node virtual">
								✏️ Write a new branch
							</a>
						</li>
						{state.data.children.map(child => <li key={child.id}>
							<StoryTreeNodeLink node={child} />
						</li>)}
					</ul>
				</div>
			</div> : <div>Loading...</div>}
		</div>;
	}

	public componentWillMount() {
		this.load(this.props.id);
	}

	public componentWillReceiveProps(props: NodePageProps) {
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
				this.setState({data});
			});
	}
}

const App = function() {
	return <div>
		<Router>
			<Route path="/nodes/:id" component={NodePage} />
			<Route path="/postNew/:parentID" component={NewPostPage} />
			<Route path="/register" component={SignupPage} />
			<Route path="/login" component={LoginPage} />
		</Router>
	</div>;
};

render(<App />, document.body);

console.log("aaaa");
