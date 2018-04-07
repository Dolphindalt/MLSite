import Route from '@ember/routing/route';

export default Route.extend({
    model(params) {
        this.store.queryRecord('news-post-model', { uuid: params.uuid }).then((res) =>{
            return res;
        }).catch((shit) => {
            console.debug(shit);
        });
    }
});
