diesel::table! {
    cards (card_id) {
        card_id -> Integer,
        company_id -> Integer,
        name -> Text,
        r#type -> Text,
        icon -> Nullable<Text>,
        color -> Nullable<Text>,
        benefits -> Nullable<Text>,
        category -> Text,
        rating -> Nullable<Integer>,
        created -> Text,
        updated -> Text,
    }
}

diesel::table! {
    companies (company_id) {
        company_id -> Integer,
        name -> Text,
        description -> Text,
        website -> Text,
        contact_email -> Text,
        created -> Text,
        updated -> Text,
    }
}

diesel::table! {
    user_cards (user_card_id) {
        user_card_id -> Integer,
        added -> Text,
        card_id -> Integer,
        expires_on -> Text,
        updated -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Integer,
        created -> Text,
        email -> Text,
        password -> Text,
        updated -> Text,
        username -> Text,
    }
}

diesel::table! {
    user_feedback (feedback_id) {
        feedback_id -> Nullable<Integer>,
        comments -> Text,
        company_id -> Integer,
        created -> Text,
        rating -> Integer,
        updated -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    vendor_deals (deal_id) {
        deal_id -> Integer,
        company_id -> Integer,
        title -> Text,
        description -> Text,
        valid_from -> Text,
        valid_to -> Text,
        created -> Text,
        updated -> Text,
    }
}

diesel::table! {
    rewards (reward_id) {
        reward_id -> Integer,
        company_id -> Integer,
        name -> Text,
        description -> Text,
        created -> Text,
        updated -> Text,
    }
}

diesel::table! {
    comments (comment_id) {
        comment_id -> Nullable<Integer>,
        comment_info -> Text,
        created -> Text,
        entity_type -> Text,
        updated -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    notifications (notification_id) {
        notification_id -> Nullable<Integer>,
        created -> Text,
        message -> Text,
        r#type -> Text,
        updated -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    user_rewards (user_reward_id) {
        user_reward_id -> Nullable<Integer>,
        added -> Text,
        expires_on -> Text,
        reward_id -> Integer,
        status -> Text,
        updated -> Text,
        user_id -> Integer,
    }
}

diesel::joinable!(cards -> companies (company_id));
diesel::joinable!(rewards -> companies (company_id));
diesel::joinable!(user_cards -> cards (card_id));
diesel::joinable!(user_cards -> users (user_id));
diesel::joinable!(user_feedback -> companies (company_id));
diesel::joinable!(user_feedback -> users (user_id));
diesel::joinable!(vendor_deals -> companies (company_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(notifications -> users (user_id));
diesel::joinable!(user_rewards -> rewards (reward_id));
diesel::joinable!(user_rewards -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cards,
    companies,
    user_cards,
    users,
    rewards,
    user_feedback,
    vendor_deals,
    comments,
    notifications,
    user_rewards
);
