import DS from 'ember-data';

export default DS.Model.extend({
    email: DS.attr('string'),
    admin: DS.attr('boolean'),
    date_created: DS.attr('string'),
    uuid: DS.attr('string'),
    staff: DS.attr('boolean'),
    rank: DS.attr('string')
});
