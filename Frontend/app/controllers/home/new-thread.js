import Controller from '@ember/controller';

export default Controller.extend({
    transitionToThreadFromController(route) {
        this.transitionToRoute(route);
        window.location.reload(true);
    }
});
