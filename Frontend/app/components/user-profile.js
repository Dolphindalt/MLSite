import Component from '@ember/component';
import { inject as service } from '@ember/service';

export default Component.extend({
    uuidToUsername: service('uuid-to-username'),
    username: '',
    modelRenderLink: '',
    willRender() {
        this.get('uuidToUsername').uuidToUsername(this.get('data.uuid')).then((user) => {
            this.set('username', user);
        });
        this.modelRenderLink = "https://visage.surgeplay.com/full/360/" + this.get('data.uuid');
    }
});
