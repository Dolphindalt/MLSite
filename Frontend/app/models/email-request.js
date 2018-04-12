import DS from 'ember-data';

export default DS.Model.extend({
    uuid: DS.attr("string"),
    email: DS.attr("string"),
    linkUuid: DS.attr("string")
});
