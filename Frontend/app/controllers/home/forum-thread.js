import Controller from '@ember/controller';

export default Controller.extend({
    actions: {
        transitionToThreadFromController(route, cat_name, thread_uuid) {
            this.transitionToRoute(route, cat_name, thread_uuid);
            window.location.reload(true);
        }
    }
});
