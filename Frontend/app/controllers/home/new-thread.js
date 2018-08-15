import Controller from '@ember/controller';

export default Controller.extend({
    actions: {
        transitionToThreadFromController(route, cat_name, chain_uuid, page) {
            this.transitionToRoute(route, cat_name, chain_uuid, page);
            window.location.reload(true);
        }
    }
});
