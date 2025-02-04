Which rust backend framework is more suitable for a simple task CRUD application.

What is this npx @rocket/create@latest as i see is some kind of frontend solution but there is no rust code there i expected this to create entire full-stack app with backend and frontend.

Explain how Rocket framework works is it MVC Pattern or ?

Are there any specification on Rust's Rocket framework. Things i need to know about lifetimes, borrowing, and other stuff ? Introduce me to the ecosystem.

Can you show me how a project structure for Rocket project should look.(not with an image show me text) I want to use MVC design pattern. First create for me a visualization of project structure with explanations how to structure my modules and where to put my code. Then i want explanation about does it have cons using MVC desing pattern in Rocket ?

What other design pattern will be better for implementing simple Crud application

Do rust have ORM features because i am working on Rocket framework and i am wondering should i use raw SQL queries or i can simplify it using Orm

i have never used ORM in Rust before i hav experience in python ORM only. Suggest me the most easy to pickup ORM tool in Rust 

How in rust rocket framework the module system works. For example in pure rust i use lib.rs and i use the other modules by the package name with mod.rs file inside. But in rocket i have created structure like src/models/task_model.rs, src/routes/routes.rs and so on.

Guide me how to work with seaOrm in Rust Rocket framework from connection setting to GET, POST request

Help me understand how migration works in seaOrm. In my case Migration created some Post{} entity for me but i need different entity i have changed it in entity folder how to work efficienlty with migrations. (Restart with new migration, Reverse..)

I WANT to remove all my migrations made and clean everything

I just deleted the folder migrations from my project now start clean and explain. I know conceptually what are migrations i need to understand the commands that are used in seaOrm to handle them. So first i need command to init the migration with my current entities that i have. 

i have migrated my migration to the db. In my entity i have defined my model with many fields but in the db i see my Task table have only 3 fields like its using different entity

But is there a way this to happen automatically. I mean when i have in src/entity/task.rs some defined model when i migrate it in the migration to have every col defined because i could rewrite the migration file but that is not optimal in the long run when i create many models

Yes but in migration i have this errorr
[E0599]: no variant or associated item named is_completed found for enum Task in the current scope
  --> src\m20250204_160342_create_tasks_table.rs:19:47
   |
19 |                     .col(ColumnDef::new(Task::is_completed).boolean().not_null().default(false))
   |                                               ^^^^^^^^^^^^ variant or associated item not found in Task
...


But it works like that i am supposed to create my migration code in hand and then use it to generate entity is that right