import GObject from 'gi://GObject';
import Adw from 'gi://Adw';

import { AboutDialog } from '../../dialogs/AboutDialog.js';

export const HomePage = GObject.registerClass({
    GTypeName: 'YdxHomePage',
    Template: 'resource:///com/keygenqt/ydx/ui/pages/home/HomePage.ui',
    InternalChildren: [
        'IdDataLabel',
        'IdFetchButton'
    ],
}, class extends Adw.NavigationPage {
    #window

    constructor(params) {
        super(params);
        this.tag = this.utils.constants.Pages.Home;
        this.#setupPageActions();
        this.#setupActions();
    }

    vfunc_realize() {
        super.vfunc_realize();
        this.#window = this.get_native();
    }

    vfunc_map() {
        super.vfunc_map();
        this._IdDataLabel.set_label(_('Fetch Data'));
    }

    #setupPageActions() {
        this.connectGroup('homePage', {
            'about': () => {
                new AboutDialog().present(this);
            },
            'logout': () => {
                this.utils.secret.clearAll();
                this.#window.navigation().switchNavigation(false);
            },
        });
    }

    #setupActions() {
        this._IdFetchButton.connect('clicked', () => {
            this.#fetchData();
        });
    }

    async #fetchData() {
        this._IdFetchButton.sensitive = false;
        this._IdDataLabel.set_label('Loading...');
        try {
            // get token and user_id
            const userId = await this.utils.secret.get(this.utils.constants.SecretKeys.UserID);
            const token = await this.utils.secret.get(this.utils.constants.SecretKeys.AccessToken);
            // query
            const response = JSON.parse(this.utils.core.request_get_user(userId, token));
            // processing
            if (!response.code || response.code === 200) {
                this._IdDataLabel.set_label(`Email: ${response.email}`);
            } else {
                this._IdDataLabel.set_label('Failed to fetch data');
                this.utils.log.error('Fetch failed:', response.message);
            }
        } catch (e) {
            this._IdDataLabel.set_label('Error fetching data');
            this.utils.log.error('Fetch error:', e);
        } finally {
            this._IdFetchButton.sensitive = true;
        }
    }
});
