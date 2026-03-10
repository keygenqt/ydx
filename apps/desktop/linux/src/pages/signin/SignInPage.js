import GObject from 'gi://GObject';
import Adw from 'gi://Adw';

export const SignInPage = GObject.registerClass({
    GTypeName: 'YdxSignInPage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/SignInPage.ui',
    InternalChildren: [],
}, class extends Adw.NavigationPage {
    #window

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.SignInPage;
    }

    vfunc_realize() {
		super.vfunc_realize();
		this.#window = this.get_native();
	}
});
