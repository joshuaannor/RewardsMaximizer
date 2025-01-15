import Model, { attr } from '@ember-data/model';

export default class RewardsModel extends Model {
  @attr('number') reward_id;
  @attr('number') company_id;
  @attr('string') name;
  @attr('string') description;
  @attr('string') created;
  @attr('string') updated;
}