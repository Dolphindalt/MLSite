import Component from '@ember/component';
import { inject as service } from '@ember/service';

export default Component.extend({
    classNames: [ "news-container" ],
    username: "",
    utu: service("uuidToUsername"),
    willRender() {
        this.get('utu').uuidToUsername(this.get('data.author')).then((user) => {
            this.set('username', user);
        }).catch(() => {
            this.set('username', "Missing No"); // hehe
        });
    }
});
