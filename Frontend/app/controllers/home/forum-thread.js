import Controller from '@ember/controller';

export default Controller.extend({
    actions: {
        transitionToThreadFromController(route) {
            this.transitionToRoute(route);
            window.location.reload(true);
        }
    }
});
