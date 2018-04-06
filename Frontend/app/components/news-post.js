import Component from '@ember/component';

export default Component.extend({
    classNames: [ "news-container" ],
    actions: {
        toPost() {
            // todo: set up single post view model
            console.debug("Boi");
            this.get('router').transitionTo("home.view-news/" + this.get('data.uuid'));
        }
    }
});
