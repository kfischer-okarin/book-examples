class WasRun
  def initialize(name)
    @name = name
    @was_run = false
  end

  def run
    send(@name)
  end

  def was_run?
    @was_run
  end

  def test_method
    @was_run = true
  end
end
