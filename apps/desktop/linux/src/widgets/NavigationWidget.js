import GObject from 'gi://GObject';
import Gtk from 'gi://Gtk';

import '../pages/welcome/WelcomePage.js';

export const NavigationWidget = GObject.registerClass({
	GTypeName: 'YdxNavigationWidget',
    Template: 'resource:///com/keygenqt/ydx/ui/widgets/NavigationWidget.ui',
	InternalChildren: [
		'IdNavigation',
	],
}, class extends Gtk.Box {
	#data = {}
	#disposable = {}

	constructor(params) {
		super(params);
	}

	params(key) {
		return this.#data[key] ?? {};
	}

	disposable(key) {
		if (this.#disposable[key]) {
			const value = this.#disposable[key];
			this.#disposable[key] = undefined;
			return value;
		}
		return undefined;
	}

	push(key, params = undefined) {
		this.#data[key] = params;
		this._IdNavigation.push_by_tag(key);
	}

	pop(
		key = undefined,
		disposable = undefined,
	) {
		if (key) {
			if (disposable) {
				this.#disposable[key] = disposable;
			}
		}
		this._IdNavigation.pop();
	}
});
