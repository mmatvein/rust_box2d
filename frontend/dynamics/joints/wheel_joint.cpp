b2Joint* World_create_wheel_joint(
    b2World* world,
    b2Body* body_a,
    b2Body* body_b,
    bool collide_connected,
    b2Vec2 local_anchor_a,
    b2Vec2 local_anchor_b,
    b2Vec2 local_axis_a,
    bool enable_motor,
    f32 max_motor_torque,
    f32 motor_speed,
    f32 stiffness,
    f32 damping
) {
    b2WheelJointDef def;
    def.bodyA = body_a;
    def.bodyB = body_b;
    def.collideConnected = collide_connected;
    def.localAnchorA = local_anchor_a;
    def.localAnchorB = local_anchor_b;
    def.localAxisA = local_axis_a;
    def.enableMotor = enable_motor;
    def.maxMotorTorque = max_motor_torque;
    def.motorSpeed = motor_speed;
    def.stiffness = stiffness;
    def.damping = damping;

    return world->CreateJoint(&def);
}

void WheelJointDef_initialize(b2WheelJointDef* self,
                              b2Body* body_a, b2Body* body_b,
                              const b2Vec2* anchor,
                              const b2Vec2* axis) {
    self->Initialize(body_a, body_b, *anchor, *axis);
}

b2Joint* WheelJoint_as_joint(b2WheelJoint* self) {
    return static_cast<b2Joint*>(self);
}
b2WheelJoint* Joint_as_wheel_joint(b2Joint* self) {
    return static_cast<b2WheelJoint*>(self);
}

const b2Vec2* WheelJoint_get_local_anchor_a(const b2WheelJoint* self) {
    return &self->GetLocalAnchorA();
}
const b2Vec2* WheelJoint_get_local_anchor_b(const b2WheelJoint* self) {
    return &self->GetLocalAnchorB();
}
const b2Vec2* WheelJoint_get_local_axis_a(const b2WheelJoint* self) {
    return &self->GetLocalAxisA();
}
f32 WheelJoint_get_joint_translation(const b2WheelJoint* self) {
    return self->GetJointTranslation();
}
f32 WheelJoint_get_joint_linear_speed(const b2WheelJoint* self) {
    return self->GetJointLinearSpeed();
}
f32 WheelJoint_get_joint_angular_speed(const b2WheelJoint* self) {
    return self->GetJointAngularSpeed();
}
bool WheelJoint_is_motor_enabled(const b2WheelJoint* self) {
    return self->IsMotorEnabled();
}
void WheelJoint_enable_motor(b2WheelJoint* self, bool flag) {
    self->EnableMotor(flag);
}
void WheelJoint_set_motor_speed(b2WheelJoint* self, f32 speed) {
    self->SetMotorSpeed(speed);
}
f32 WheelJoint_get_motor_speed(const b2WheelJoint* self) {
    return self->GetMotorSpeed();
}
void WheelJoint_set_max_motor_torque(b2WheelJoint* self, f32 torque) {
    self->SetMaxMotorTorque(torque);
}
f32 WheelJoint_get_max_motor_torque(const b2WheelJoint* self) {
    return self->GetMaxMotorTorque();
}
f32 WheelJoint_get_motor_torque(const b2WheelJoint* self, f32 inv_dt) {
    return self->GetMotorTorque(inv_dt);
}
void WheelJoint_set_stiffness(b2WheelJoint* self, f32 stiffness) {
    self->SetStiffness(stiffness);
}
f32 WheelJoint_get__stiffness(const b2WheelJoint* self) {
    return self->GetStiffness();
}
void WheelJoint_set_damping(b2WheelJoint* self, f32 ratio) {
    self->SetDamping(ratio);
}
f32 WheelJoint_get_damping(const b2WheelJoint* self) {
    return self->GetDamping();
}
