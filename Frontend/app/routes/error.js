import Route from '@ember/routing/route';

export default Route.extend({
    model() {
        return { status: this.get('status'), msg: this.get('msg') };
    },
    status: 200,
    msg: "",
    actions: {
        e404() {
            this.set('status', 404);
            this.set('msg', "The resource you are looking for cannot be found.");
        },
        e403() {
            this.set('status', 403);
            this.set('msg', "Access denied.");
        }
    }
});
